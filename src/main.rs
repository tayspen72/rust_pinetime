//==============================================================================
// Notes
//==============================================================================
// main.rs

//==============================================================================
// Crates and Mods
//==============================================================================
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _; // Breakpoint on `rust_begin_unwind` to catch panics

mod config;
mod drivers;
use drivers::*;
mod mcu;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================


//==============================================================================
// Variables
//==============================================================================


//==============================================================================
// Main
//==============================================================================
#[entry]
fn main() -> ! {

	init();
	
	let device_info = app::DeviceInfo::take().unwrap();

	loop {
		task_handler(&device_info);
	};
}

//==============================================================================
// Private Functions
//==============================================================================
fn init() {
	mcu::init(mcu::rtc::WakeInterval::Interval250MS);

	lcd::lcd_api::init();
	debug::init();
	
	button::init();
	// touch::init(p);
}

//==============================================================================
// Task Handler
//==============================================================================
fn task_handler(d: &app::DeviceInfo) {
	mcu::task_handler(d);
	
	debug::task_handler(d);
	// button::task_handler();
	// lcd::lcd_api::task_handler();
	// touch::task_handler();

	app::task_handler();
}
