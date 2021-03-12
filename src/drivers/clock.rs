//==============================================================================
// Notes
//==============================================================================
// drivers::clock.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use core::cell::Cell;
use cortex_m::interrupt::{free, Mutex};
use crate::drivers::app;
use crate::mcu::rtc;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[derive(Clone, Copy)]
pub struct Time {
	pub hours: u8,
	pub minutes: u8,
	pub seconds: u8
}

//==============================================================================
// Variables
//==============================================================================
static TIME: Mutex<Cell<Time>> = Mutex::new(Cell::new( Time {
	hours: 0,
	minutes: 0,
	seconds: 0
}));


//==============================================================================
// Public Functions
//==============================================================================
pub fn init() {

}

//==============================================================================
// Private Functions
//==============================================================================
fn update_add_second() {
	free(|cs| {
		let mut time = TIME.borrow(cs).get();
		time.seconds += 1;

		if time.seconds >= 60 {
			time.seconds = 0;
			time.minutes += 1;
		
			if time.minutes >= 60 {
				time.minutes = 0;
				time.hours += 1;
				
				if time.hours >= 24 {
					time.hours = 0;
				}
			}
		}

		TIME.borrow(cs).set(time);
	});
}

//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler(d: &mut app::DeviceInfo) {
	static mut LAST_TIMESTAMP: u32 = 0;

	unsafe {
		if rtc::get_timediff(LAST_TIMESTAMP) >= 1 {
			LAST_TIMESTAMP = rtc::get_timestamp();
			update_add_second();
			d.time = free(|cs| TIME.borrow(cs).get());
		}
	}

}