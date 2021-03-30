//==============================================================================
// Notes
//==============================================================================
// app::mod.rs

//==============================================================================
// Crates and Mods
//==============================================================================
pub mod app;
pub mod display;
mod icon;
pub mod info;
pub mod page;

use cortex_m::asm::wfi;
use super::drivers;
use super::mcu;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================


//==============================================================================
// Variables
//==============================================================================


//==============================================================================
// Public Functions
//==============================================================================


//==============================================================================
// Private Functions
//==============================================================================
fn get_unhandled_flags(flags: &info::DeviceInfoChangeFlags) -> bool {
	if flags.battery_voltage ||
		flags.button_press ||
		flags.charger_state ||
		flags.display_state ||
		flags.time_change ||
		flags.touch_event {
			true
	}
	else {
		false
	}
}

//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler(d: &mut info::DeviceInfo) {
	// Handle any pending tasks on the current page
	app::task_handler(d);

	let app_busy = get_unhandled_flags(&d.change_flags);
	let drivers_busy = 
		if let drivers::DriversState::Idle = drivers::get_busy(){ false } else { true };
	let mcu_busy = 
		if let mcu::McuState::Idle = mcu::get_busy() { false } else { true };

	// If nothing is busy, sleep
	if !app_busy && !drivers_busy && !mcu_busy {
		wfi();
	}
}