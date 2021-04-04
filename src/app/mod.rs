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
fn clear_flags(d: &mut info::DeviceInfo) {
	if d.change_flags.display_state {
		d.change_flags.display_state = false;
	}
}

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

fn get_busy(d: &mut info::DeviceInfo) -> bool {
	// Check if ready to sleep:
	//	- Unhandled DeviceInfo changes
	//	- Driver events that need processing
	// 	- MCU events that need processing
	if get_unhandled_flags(&d.change_flags) {
		return true;
	}

	if let drivers::DriversState::Idle = drivers::get_busy() { 
		return true;
	}

	if let mcu::McuState::Idle = mcu::get_busy() {
		return true;
	}
	false
}

//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler(d: &mut info::DeviceInfo) {
	// First, clear all previously set flags
	clear_flags(d);

	// Call the task handler for the current page
	match d.app_page {
		page::AppPage::Home => page::home::task_handler(d),
		page::AppPage::Log => page::log::task_handler(d),
		page::AppPage::Notifications => page::notifications::task_handler(d),
		page::AppPage::Settings => page::settings::task_handler(d),
		page::AppPage::Startup => {
			page::startup::print_page();
			d.app_page = page::AppPage::Home;
			page::change_page(d);
		},
	}

	// if d.change_flags.display_state {
	// 	match d.display_state {
	// 		DisplayState::Dim => drivers::lcd::lcd_api::set_backlight(drivers::lcd::lcd_api::BacklightBrightness::Brightness1),
	// 		DisplayState::Off => drivers::lcd::lcd_api::set_backlight(drivers::lcd::lcd_api::BacklightBrightness::Brightness0),
	// 		DisplayState::On => drivers::lcd::lcd_api::set_backlight(drivers::lcd::lcd_api::BacklightBrightness::Brightness7),
	// 	}
	// }

	// If nothing is busy, sleep
	if !get_busy(d) {
		wfi();
	}
}
