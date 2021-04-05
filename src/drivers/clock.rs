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

pub fn update_time(is_military_time: bool) {
	write(get_digits(is_military_time), false);
}

pub fn write_time(is_military_time: bool) {
	write(get_digits(is_military_time), true);
}

//==============================================================================
// Private Functions
//==============================================================================
fn get_digits(is_military_time: bool) -> [u8; 4] {
	let time = free(|cs| TIME.borrow(cs).get());
	let mut hours = time.hours;

	// Bring in militart time flag as needed
	if !is_military_time {
		hours = hours % 12;
		if hours == 0 {
			hours = 12;
		}
	}

	// Return each digit
	[
		(hours/10)%10,
		hours%10,
		(time.minutes/10)%10,
		time.minutes%10,
	]
}

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

fn write(digits: [u8; 4], force_update: bool) {
	unsafe {
		for i in 0..4 {
			if force_update || digits[i] != DIGITS_ON_DISPLAY[i] {
				DIGITS_ON_DISPLAY[i] = digits[i];
				if i > 0 || digits[i] > 0 {
					lcd::font::write_time_character(digits[i], DIGITS_X[i], DIGITS_Y[i], lcd::lcd_api::Color::Blue, lcd::lcd_api::Color::Black);
				}
			}
		}
	}
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