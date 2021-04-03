//==============================================================================
// Notes
//==============================================================================
// drivers::clock.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use core::cell::Cell;
use cortex_m::interrupt::{free, Mutex};
use crate::app::info;
use crate::drivers::lcd;
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
static mut DIGITS_ON_DISPLAY: [u8; 6] = [10; 6];	// Init at 10 to force write the first time
static TIME: Mutex<Cell<Time>> = Mutex::new(Cell::new( Time {
	hours: 0,
	minutes: 0,
	seconds: 0
}));

const DIGITS_X: [u16; 4] = [ 69, 109, 159, 199];
const DIGITS_Y: [u16; 4] = [ 0; 4 ];


//==============================================================================
// Public Functions
//==============================================================================
pub fn init() {
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

pub fn update_time() {
	let time = free(|cs| TIME.borrow(cs).get());
	let digits: [u8; 4] = [
		(time.hours/10)%10,
		time.hours%10,
		(time.minutes/10)%10,
		time.minutes%10,
	];

	unsafe {
		for i in 0..4 {
			if digits[i] != DIGITS_ON_DISPLAY[i] {
				DIGITS_ON_DISPLAY[i] = digits[i];
				lcd::font::write_time_character(digits[i], DIGITS_X[i], DIGITS_Y[i], lcd::lcd_api::Color::Blue, lcd::lcd_api::Color::Black);
			}
		}
	}
}

pub fn write_time() {
	let time = free(|cs| TIME.borrow(cs).get());
	let digits: [u8; 4] = [
		(time.hours/10)%10,
		time.hours%10,
		(time.minutes/10)%10,
		time.minutes%10,
	];

	unsafe {
		for i in 0..4 {
			DIGITS_ON_DISPLAY[i] = digits[i];
			lcd::font::write_time_character(digits[i], DIGITS_X[i], DIGITS_Y[i], lcd::lcd_api::Color::Blue, lcd::lcd_api::Color::Black);
		}
	}
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
pub fn task_handler(d: &mut info::DeviceInfo) {
	static mut LAST_TIMESTAMP: u32 = 0;

	if d.change_flags.time_change {
		d.change_flags.time_change = false;
	}

	unsafe {
		if rtc::get_timediff(LAST_TIMESTAMP) >= 1 {
			LAST_TIMESTAMP = rtc::get_timestamp();
			update_add_second();
			d.change_flags.time_change = true;
			d.time = free(|cs| TIME.borrow(cs).get());
		}
	}
}