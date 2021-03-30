//==============================================================================
// Notes
//==============================================================================
// app::mod.rs

//==============================================================================
// Crates and Mods
//==============================================================================
pub mod info;
pub mod page;

use cortex_m::asm::wfi;
use super::drivers;
use super::mcu;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[allow(dead_code)]
pub enum DisplayState{
	Off, 
	Dim,
	On
}

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
	if flags.app_page ||
		flags.battery_voltage ||
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
// Handle all input events and route them to pages as needed
	match d.app_page {
		page::AppPage::Home => page::home::event_handler(d),
		page::AppPage::Log => page::log::event_handler(d),
		page::AppPage::Notifications => page::notifications::event_handler(d),
		page::AppPage::Settings => page::settings::event_handler(d),
		page::AppPage::Startup => {
			// If in startup, change to home
			d.app_page = page::AppPage::Home;
			d.change_flags.app_page = true;
		},
	}
	// Check if ready to sleep:
	//	- Unhandled DeviceInfo changes
	//	- Driver events that need processing
	// 	- MCU events that need processing
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
