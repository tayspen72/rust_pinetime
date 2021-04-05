//==============================================================================
// Notes
//==============================================================================
// app::page::mod.rs

//==============================================================================
// Crates and Mods
//==============================================================================
pub mod home;
pub mod log;
pub mod notifications;
pub mod settings;
pub mod startup;

use crate::drivers::lcd::lcd_api;
use super::info;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[allow(dead_code)]
pub enum AppPage {
	Home,
	Notifications,
	Log,
	Settings,
	Startup,
}

//==============================================================================
// Variables
//==============================================================================


//==============================================================================
// Public Functions
//==============================================================================
pub fn change_page(d: &mut info::DeviceInfo) {
	// Clear what was there
	lcd_api::set_backlight(lcd_api::BacklightBrightness::Brightness0);
	lcd_api::fill_background(lcd_api::Color::Black);

	match d.app_page {
		AppPage::Home => home::start_page(d),
		AppPage::Notifications => notifications::start_page(),
		AppPage::Log => log::start_page(),
		AppPage::Settings => settings::start_page(d),
		_ => (),
	}

	// Restore brightness to show new page
	lcd_api::set_backlight(lcd_api::BacklightBrightness::Brightness4);
}

//==============================================================================
// Private Functions
//==============================================================================


//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
