//==============================================================================
// Notes
//==============================================================================
// mcu::timer.rs
// For now, this library is designed as a blocking delay function with 
// millisecond precision.

//==============================================================================
// Crates and Mods
//==============================================================================
use nrf52832_pac::interrupt;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================


//==============================================================================
// Macros
//==============================================================================


//==============================================================================
// Variables
//==============================================================================
static mut _INITIALIZED: bool = false;
static mut _TIMER_RUNNING: bool = false;
static mut _TIMER_COUNT: u32 = 0;

//==============================================================================
// Implementations
//==============================================================================
pub fn init(p: &nrf52832_pac::Peripherals) {
	if unsafe { _INITIALIZED } {
		return;
	}

	let t = &p.TIMER0;

	nrf52832_pac::NVIC::mask(nrf52832_pac::Interrupt::TIMER0);

	// Stop the timer before init for good measure 
	t.tasks_stop.write(|w| unsafe{ w.bits(1) });
	t.tasks_clear.write(|w| unsafe { w.bits(1) });
	
	// Define timer mode and config
	t.mode.write(|w| w.mode().timer());
	t.bitmode.write(|w| w.bitmode()._16bit());
	// Use prescaler 4: 16MHz / 2^7 = 125kHz-> will force 1MHz low freq clock for better power usage
	t.prescaler.write(|w| unsafe { w.prescaler().bits(0x7) });

	//Enable timer interrupts on compare 0 overflow
	t.intenset.write(|w| w.compare0().set_bit());

	unsafe {
		nrf52832_pac::NVIC::unpend(nrf52832_pac::Interrupt::TIMER0);
		nrf52832_pac::NVIC::unmask(nrf52832_pac::Interrupt::TIMER0);
	}

	unsafe {
		_TIMER_RUNNING = false;
		_INITIALIZED = true;
	}
}

pub fn delay(p: &nrf52832_pac::Peripherals, milliseconds: u32) {
	if unsafe { !_INITIALIZED } {
		init(p);
	}
	
	let mut current_count = unsafe { _TIMER_COUNT };
	let target_count = current_count + milliseconds;

	start(p);

	while current_count < target_count {
		// task_handler(p);
		cortex_m::interrupt::free(|_| {
			current_count = unsafe { _TIMER_COUNT };
		});
	}

	stop(p);
}

fn enable(p: &nrf52832_pac::Peripherals, is_enabled: bool) {
	if unsafe { !_INITIALIZED } {
		init(p);
	}
	
	let t = &p.TIMER0;

	// Stop the timer before config 
	t.tasks_stop.write(|w| unsafe{ w.bits(1) });
	t.tasks_clear.write(|w| unsafe { w.bits(1) });

	unsafe { _TIMER_RUNNING = false; }

	//configure the timer to repeat indefinitely until stopped
	t.shorts.write(|w| w 
		.compare0_stop().disabled()
		.compare0_clear().enabled()
	);

	// Configure the timer to fire interrupt in 1ms intervals
	t.cc[0].write(|w| unsafe { w.bits(125) });

	if is_enabled {
		t.tasks_start.write(|w| unsafe { w.bits(1) });
		unsafe { _TIMER_RUNNING = true; }
	}
}

fn start(p: &nrf52832_pac::Peripherals) {
	if unsafe { _TIMER_RUNNING } {
		return; 
	}
	enable(p, true);
}

fn stop(p: &nrf52832_pac::Peripherals) {
	enable(p, false);
	unsafe { _TIMER_COUNT = 0 };
	unsafe { _TIMER_RUNNING = false; };
}

//==============================================================================
// Interrupt Handler
//==============================================================================
#[interrupt]
fn TIMER0() {
	let t = unsafe { &nrf52832_pac::Peripherals::steal().TIMER0 };
	if t.events_compare[0].read().bits() > 0 {
		t.events_compare[0].write(|w| unsafe { w.bits(0) });
		unsafe { _TIMER_COUNT += 1; };
	}
}

//==============================================================================
// Task Handler
//==============================================================================
#[allow(dead_code)]
pub fn task_handler(p: &nrf52832_pac::Peripherals) {
	cortex_m::interrupt::free(|_| {

	});

	if unsafe { _TIMER_RUNNING } {
		stop(p);
	}

}
