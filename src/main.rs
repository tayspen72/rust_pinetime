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

mod app;
use app::info;
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
	
	let mut device_info = info::DeviceInfo::take().unwrap();

	loop {
		task_handler(&mut device_info);
	};
}

//==============================================================================
// Private Functions
//==============================================================================
fn init() {
	mcu::init(mcu::rtc::WakeInterval::Interval250MS);

	lcd::lcd_api::init();
	debug::init();
	
	battery::init();
	button::init();
	clock::init();
	touch::init();
}

//==============================================================================
// Task Handler
//==============================================================================
fn task_handler(d: &mut info::DeviceInfo) {
	mcu::task_handler(d);
	
	debug::task_handler(d);
	battery::task_handler(d);
	button::task_handler(d);
	clock::task_handler(d);
	// lcd::lcd_api::task_handler();
	touch::task_handler(d);

	app::task_handler();
}
