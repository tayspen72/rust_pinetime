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
use nrf52832_pac;
use panic_halt as _; // Breakpoint on `rust_begin_unwind` to catch panics

mod config;
mod drivers;
use drivers::*;
mod mcu;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================


//==============================================================================
// Macros
//==============================================================================


//==============================================================================
// Variables
//==============================================================================


//==============================================================================
// Implementations
//==============================================================================
#[entry]
fn main() -> ! {
	let peripherals = nrf52832_pac::Peripherals::take().unwrap();

	app_init(&peripherals);
	
	let device_info = app::app::DeviceInfo::take().unwrap();

	loop {
		app_task_handler(&peripherals, &device_info);
	};
}

fn app_init(p: &nrf52832_pac::Peripherals) {
	mcu::rtc::init(p, mcu::rtc::WakeInterval::Interval250MS);

	lcd::lcd_api::init(p);
	debug::init(p);
	
	button::init(p);
	touch::init(p);
}

//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
// TODO: This will be developed into passing around some device flags structure to handle changes as needed
fn app_task_handler(p: &nrf52832_pac::Peripherals, d: &app::app::DeviceInfo) {
	debug::task_handler(p, d);
	button::task_handler(p);
	lcd::lcd_api::task_handler();
	// touch::task_handler();
	mcu::rtc::task_handler();
}
