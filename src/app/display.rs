//==============================================================================
// Notes
//==============================================================================
// app::display.rs
// Display state enum and top level handling

//==============================================================================
// Crates and Mods
//==============================================================================
use super::{icon, info, page};
use crate::drivers::lcd::{font, lcd_api};

//==============================================================================
// Enums, Structs, and Types
//==============================================================================


//==============================================================================
// Variables
//==============================================================================
#[allow(dead_code)]
pub enum DisplayState {
	On,
	Dim,
	Off
}

//==============================================================================
// Public Functions
//==============================================================================


//==============================================================================
// Private Functions
//==============================================================================
pub fn write_battery_level(level: u8, voltage: u16, charging: bool) {
	// Draw outline
	lcd_api::fill_rectangle(icon::BATTERY_ICON_X + 7, 29, icon::BATTERY_ICON_Y, 15, icon::BATTERY_OUTLINE_COLOR);

	// Fill in the middle of the battery
	let color = if charging { icon::CHARGER_COLOR } else { icon::BATTERY_FILL_COLOR };
	lcd_api::fill_rectangle(icon::BATTERY_ICON_X + 9, 25, icon::BATTERY_ICON_Y + 2, 11, color);

	// Print out battery level blocks
	for l in 0..level {
		let x = icon::BATTERY_ICON_X + 10 + (l as u16 * 6);
		lcd_api::fill_rectangle(x, 5, icon::BATTERY_ICON_Y + 3, 9, icon::BATTERY_OUTLINE_COLOR);
	}

	// Write out the voltage value
	let mut divider: u16 = 1000;
	let color = if charging { icon::CHARGER_COLOR } else { icon::BATTERY_OUTLINE_COLOR };
	for d in 0..4 {
		let v: u8 = if d == 1 {
			0x2E
		}
		else {
			divider = divider / 10;
			((voltage / divider) % 10) as u8
		};
		let x = icon::BATTERY_ICON_X + 8 + (d as u16 * 6);
		
		font::write_minimal_character((0x30 + v) as char, x, icon::BATTERY_ICON_Y + 16, color, icon::BATTERY_FILL_COLOR, 1)
	}

	// Print (or remove) the charger status icon
	let color = if charging { icon::CHARGER_COLOR } else { icon::BATTERY_FILL_COLOR };
	lcd_api::fill_rectangle(icon::BATTERY_ICON_X, 7, icon::BATTERY_ICON_Y, 15, color);
	if charging {
		lcd_api::fill_rectangle(icon::BATTERY_ICON_X, 4, icon::BATTERY_ICON_Y, 6, icon::BATTERY_FILL_COLOR);
		lcd_api::fill_rectangle(icon::BATTERY_ICON_X + 2, 2, icon::BATTERY_ICON_Y + 9, 6, icon::BATTERY_FILL_COLOR);
	}
	lcd_api::fill_rectangle(icon::BATTERY_ICON_X + 5, 2, icon::BATTERY_ICON_Y + 3, 9, icon::BATTERY_OUTLINE_COLOR);
}

//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler(d: &mut info::DeviceInfo) {
	// If any pending flags, clear them now
	if d.change_flags.display_state {
		d.change_flags.display_state = false;
	}

	// Update display for all newly changes flags
	if d.change_flags.battery_voltage || d.change_flags.charger_state {
		// Only update battery voltage when display is on and on home page
		if let DisplayState::On = d.display_state {
			if let page::AppPage::Home = d.app_page {
				write_battery_level(d.battery_level as u8, d.battery_voltage, d.flags.charger_connected);
			}
		}
	}

	match d.display_state {
		DisplayState::On => (),
		DisplayState::Dim => (),
		_ => ()
	}
}