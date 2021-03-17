//==============================================================================
// Notes
//==============================================================================
// mcu::timer.rs
// For now, this library is designed as a blocking delay function with 
// millisecond precision.

//==============================================================================
// Crates and Mods
//==============================================================================
use core::cell::{Cell, RefCell};
use core::ops::DerefMut;
use cortex_m::interrupt::{free, Mutex};
use nrf52832_pac::interrupt;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================


//==============================================================================
// Variables
//==============================================================================
const TIMER_MULTIPLIER: u32 = 10;

static TIMER_COUNT: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));
static TIMER_RUNNING: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

static TIMER_HANDLE: Mutex<RefCell<Option<nrf52832_pac::TIMER0>>> = 
	Mutex::new(RefCell::new(None));

//==============================================================================
// Public Functions
//==============================================================================
pub fn init(timer: nrf52832_pac::TIMER0) {

	configure(&timer);

	free(|cs| TIMER_HANDLE.borrow(cs).replace(Some(timer)));
}

pub fn get_busy() -> bool {
	if free(|cs| TIMER_RUNNING.borrow(cs).get()) {
		return true;
	}

	false
}

pub fn delay(milliseconds: u32) {	
	let mut current_count = free(|cs| TIMER_COUNT.borrow(cs).get());
	let target_count = current_count + milliseconds;

	start();

	while current_count < target_count {
		current_count = free(|cs| TIMER_COUNT.borrow(cs).get());
	}

	stop();
}

//==============================================================================
// Private Functions
//==============================================================================
fn configure (timer: &nrf52832_pac::TIMER0) {
	nrf52832_pac::NVIC::mask(nrf52832_pac::Interrupt::TIMER0);

	// Stop the timer before init for good measure 
	timer.tasks_stop.write(|w| unsafe{ w.bits(1) });
	timer.tasks_clear.write(|w| unsafe { w.bits(1) });
	
	// Define timer mode and config
	timer.mode.write(|w| w.mode().timer());
	timer.bitmode.write(|w| w.bitmode()._16bit());
	// Use prescaler 4: 16MHz / 2^7 = 125kHz-> will force 1MHz low freq clock for better power usage
	timer.prescaler.write(|w| unsafe { w.prescaler().bits(0x7) });

	//Enable timer interrupts on compare 0 overflow
	timer.intenset.write(|w| w.compare0().set_bit());

	unsafe {
		nrf52832_pac::NVIC::unpend(nrf52832_pac::Interrupt::TIMER0);
		nrf52832_pac::NVIC::unmask(nrf52832_pac::Interrupt::TIMER0);
	}

	free(|cs| TIMER_RUNNING.borrow(cs).set(false));
}

fn enable(is_enabled: bool) {
	free(|cs| {
		if let Some(ref mut timer) = TIMER_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			// Stop the timer before config 
			timer.tasks_stop.write(|w| unsafe{ w.bits(1) });
			timer.tasks_clear.write(|w| unsafe { w.bits(1) });

			free(|cs| TIMER_RUNNING.borrow(cs).set(false));

			//configure the timer to repeat indefinitely until stopped
			timer.shorts.write(|w| w 
				.compare0_stop().disabled()
				.compare0_clear().enabled()
			);

			// Configure the timer to fire interrupt in 1ms intervals
			const HIT_INTERVAL: u32 = 125 * TIMER_MULTIPLIER; // 125kHz refclk
			timer.cc[0].write(|w| unsafe { w.bits(HIT_INTERVAL) });

			if is_enabled {
				timer.tasks_start.write(|w| unsafe { w.bits(1) });
				free(|cs| TIMER_RUNNING.borrow(cs).set(true));
			}
		}
	});
}

fn start() {
	enable(true);
}

fn stop() {
	enable(false);
	free(|cs| TIMER_COUNT.borrow(cs).set(0));
	free(|cs| TIMER_RUNNING.borrow(cs).set(false));
}

//==============================================================================
// Interrupt Handler
//==============================================================================
#[interrupt]
fn TIMER0() {
	let timer_hit: bool = free(|cs| {
		if let Some(ref mut timer) = TIMER_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			if timer.events_compare[0].read().bits() > 0 {
				timer.events_compare[0].write(|w| unsafe { w.bits(0) });
				true
			}
			else {
				false
			}
		}
		else {
			false
		}
	});

	if timer_hit {
		free(|cs| TIMER_COUNT.borrow(cs).set(TIMER_COUNT.borrow(cs).get() + TIMER_MULTIPLIER));
	}
}

//==============================================================================
// Task Handler
//==============================================================================

