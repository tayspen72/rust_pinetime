//==============================================================================
// Notes
//==============================================================================
// mcu::rtc.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use nrf52832_pac;
use nrf52832_pac::interrupt;
use crate::drivers::debug;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[allow(dead_code)]
pub enum WakeInterval {
	Interval125MS	= 512,
	Interval250MS	= 1024,
	Interval500MS	= 2048,
	Interval1S 		= 4096
}

//==============================================================================
// Macros
//==============================================================================


//==============================================================================
// Variables
//==============================================================================
static mut _INITIALIZED: bool = false;
static mut _WAKE_INTERVAL: u32 = 0;
static mut _FRACTION: u32 = 0;
static mut _SECONDS: u32 = 0;

//==============================================================================
// Implementations
//==============================================================================
#[allow(dead_code)]
pub fn init(p: &nrf52832_pac::Peripherals, interval: WakeInterval) {
	if unsafe { _INITIALIZED } {
		return;
	}

	unsafe { _WAKE_INTERVAL = interval as u32; }

	enable(p, true);

	unsafe { _INITIALIZED = true; }
}

#[allow(dead_code)]
pub fn get_timestamp() -> u32 {
	unsafe { _SECONDS }
}

#[allow(dead_code)]
pub fn get_timestamp_fraction() -> u32 {
	unsafe { _FRACTION }
}

fn enable(p: &nrf52832_pac::Peripherals, is_enabled: bool) {
	let clock = &p.CLOCK;
	let rtc = &p.RTC0;

	nrf52832_pac::NVIC::mask(nrf52832_pac::Interrupt::RTC0);

	if is_enabled {
		// Enable XTAL for Low Freq Clock Source
		clock.lfclksrc.write(|w| w.src().xtal());
		clock.tasks_lfclkstart.write(|w| unsafe { w.bits(1) });
		//TODO: waiting indefinitely here
		while clock.events_lfclkstarted.read().bits() == 0 {};

		//Disable RTC
		rtc.tasks_stop.write(|w| unsafe { w.bits(1) });
		
		//prescale by 8 : 32768 / 8 = 4096 Hz
		rtc.prescaler.write(|w| unsafe { w.prescaler().bits(7) });
	

		//define the wake interval
		rtc.cc[0].write(|w| unsafe { w.bits(_WAKE_INTERVAL) });

		//connect the interrupt event signal on compare0 match
		rtc.intenset.write(|w| w.compare0().set_bit());

		unsafe {
			nrf52832_pac::NVIC::unpend(nrf52832_pac::Interrupt::RTC0);
			nrf52832_pac::NVIC::unmask(nrf52832_pac::Interrupt::RTC0);
		}

		//Enable RTC
		rtc.tasks_start.write(|w| unsafe { w.bits(1) });
	}

	unsafe { 
		_SECONDS = 0;
		_FRACTION = 0;
	}
}

// =============================================================================
// Interrupt Handler
//==============================================================================
#[interrupt]
fn RTC0() {
	unsafe { 
		let rtc = nrf52832_pac::Peripherals::steal().RTC0;
	
		if rtc.events_compare[0].read().bits() > 0 {
			_FRACTION += _WAKE_INTERVAL;
		
			if _FRACTION >= WakeInterval::Interval1S as u32 {
				_SECONDS += _FRACTION / WakeInterval::Interval1S as u32;
				_FRACTION = 0;
			}
		}

		rtc.events_compare[0].write(|w| w.bits(0));
		rtc.tasks_clear.write(|w| w.bits(0));
	}
}

//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler(){
	static mut LAST_SECONDS: u32 = 0;
	
	unsafe {
		if LAST_SECONDS != get_timestamp() {
			LAST_SECONDS = get_timestamp();
			// let time = "Time: ".as_bytes() + debug::number_to_string(&last_seconds);
			debug::push_log("Next second: ");
		}
	}
}