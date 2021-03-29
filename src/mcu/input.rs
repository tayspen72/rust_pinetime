//==============================================================================
// Notes
//==============================================================================
// mcu::input.rs
// Watcher and handler for a GPIO pin defined as an input

//==============================================================================
// Crates and Mods
//==============================================================================
use core::cell::{Cell, RefCell};
use core::ops::DerefMut;
use cortex_m::interrupt::{free, Mutex};
use nrf52832_pac::interrupt;
use super::gpio;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[derive(Clone, Copy)]
struct InputQueueEntry{
	pin: Option<u8>,
	callback: &'static dyn Fn(),
	real_time_callback: bool
}

#[derive(Clone, Copy)]
pub struct PinConfig {
	pub pin: u8,
	pub polarity: nrf52832_pac::gpiote::config::POLARITY_A,
	pub pull: nrf52832_pac::p0::pin_cnf::PULL_A,
	pub callback: &'static dyn Fn(),
	pub real_time_callback: bool
}

//==============================================================================
// Variables
//==============================================================================
const EVENT_LEN: usize = 8;
const INPUT_QUEUE_LEN: usize = 16;
static mut EVENT_MAP: [InputQueueEntry; EVENT_LEN] = [
	InputQueueEntry { pin: None, callback: &dummy_function, real_time_callback: false }; EVENT_LEN];
static HEAD: Mutex<Cell<usize>> = Mutex::new(Cell::new(0));
static TAIL: Mutex<Cell<usize>> = Mutex::new(Cell::new(0));
static QUEUE: Mutex<Cell<[u8; INPUT_QUEUE_LEN]>> = Mutex::new(Cell::new([0; INPUT_QUEUE_LEN]));
static GPIOTE_HANDLE: Mutex<RefCell<Option<nrf52832_pac::GPIOTE>>> = 
	Mutex::new(RefCell::new(None));

//==============================================================================
// Public Functions
//==============================================================================
pub fn init(gpiote: nrf52832_pac::GPIOTE) {
	free(|cs| GPIOTE_HANDLE.borrow(cs).replace(Some(gpiote)));
}

pub fn init_pin(config: PinConfig) {
	// If pin is already configured, quit
	if get_event_exists(config.pin) {
		return;
	}

	// Get the next available interrupt index
	if let Some(event) = get_event_index(config.pin) {
		configure(config, event);
		
		// Assign this pin config to the event map
		unsafe {
			EVENT_MAP[event].pin = Some(config.pin);
			EVENT_MAP[event].callback = config.callback;
			EVENT_MAP[event].real_time_callback = config.real_time_callback;
		}
	}
}

//==============================================================================
// Private Functions
//==============================================================================
fn configure(config: PinConfig, event: usize) {
	// Set pin config
	gpio::pin_setup(config.pin, nrf52832_pac::p0::pin_cnf::DIR_A::INPUT, gpio::PinState::PinLow, config.pull);

	// Pause interrupts during config
	nrf52832_pac::NVIC::mask(nrf52832_pac::Interrupt::GPIOTE);

	free(|cs| {
		if let Some(ref mut gpiote) = GPIOTE_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			// Assign the pin config to the event
			gpiote.config[event].write(|w| unsafe { w
				.polarity().variant(config.polarity)
				.psel().bits(config.pin)
				.mode().event()
			});

			// Enable the interrupt
			gpiote.intenset.modify(|_r, w| unsafe { w.bits(1 << event) });

			// Just in case, clear any pending events that need handling
			gpiote.events_in[event].modify(|_r, w| unsafe { w.bits(0) });
		}
	});

	unsafe {
		nrf52832_pac::NVIC::unpend(nrf52832_pac::Interrupt::GPIOTE);
		nrf52832_pac::NVIC::unmask(nrf52832_pac::Interrupt::GPIOTE);
	}
}

fn dummy_function() {}

fn get_event_exists(pin: u8) -> bool {
	// Returns true if the pin is already in the queue
	unsafe {
		for e in 0..EVENT_LEN {
			if let Some(tmp_pin) = EVENT_MAP[e].pin {
				if tmp_pin == pin {
					return true;
				}
			}
		}
		false
	}
}

fn get_event_index(pin: u8) -> Option<usize> {
	// Returns the index of the pin in the queue
	// If the pin does not have an index, one will be assigned and returned
	unsafe {
		for e in 0..EVENT_LEN {
			if let Some(tmp_pin) = EVENT_MAP[e].pin {
				if tmp_pin == pin {
					return Some(e);
				}
			}
			else {
				return Some(e);
			}
		}
		return None;
	}
}

//==============================================================================
// Interrupt Handler
//==============================================================================
#[interrupt]
fn GPIOTE() {
	let mut event: Option<usize> = None;

	free(|cs| {
		if let Some(ref mut gpiote) = GPIOTE_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			// Find the event that caused the interrupt
			for e in 0..EVENT_LEN {
				if gpiote.events_in[e].read().bits() != 0 {
					// Clear the event flag
					gpiote.events_in[e].write(|w| unsafe { w.bits(0) });
					event = Some(e);
					break;	
				}
			}
		}
	});

	// Safe this event in the queue
	if let Some(e) = event {
		// If real time callback enabled, run the handler immediately and return
		unsafe {
			if EVENT_MAP[e].real_time_callback {
				let f = EVENT_MAP[e].callback;
				f();
				return;
			}
		}

		// grab the current tail position
		let tail: usize = free(|cs| TAIL.borrow(cs).get());
		
		// Update the tail pointer to the next available position
		free(|cs| TAIL.borrow(cs).set( {
			let mut tail = TAIL.borrow(cs).get() + 1;
			if tail == EVENT_LEN {
				tail = 0;
			}
			tail
		}));

		// Push this event onto the queue
		free(|cs| { 
			let mut queue = QUEUE.borrow(cs).get();
			queue[tail] = e as u8;
			QUEUE.borrow(cs).set(queue);
		})
	}
}

//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler() {
	let mut head: usize = free(|cs| HEAD.borrow(cs).get());
	let tail: usize = free(|cs| TAIL.borrow(cs).get());

	while head != tail {
		let event = free(|cs| QUEUE.borrow(cs).get())[head] as usize;

		unsafe { 
			let f = EVENT_MAP[event].callback;
			f();
		}

		// Increment the head pointer
		head = head + 1;
		if head == EVENT_LEN {
			head = 0;
		}
	}

	// Reset the head pointer
	free(|cs| HEAD.borrow(cs).set(head));
}