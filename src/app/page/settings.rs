//==============================================================================
// Notes
//==============================================================================
// app::page::settings.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use crate::app::{info, page};
use crate::drivers::lcd::{font, lcd_api};
use crate::drivers::touch::Gesture;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================


//==============================================================================
// Variables
//==============================================================================
// Battery and Charger
pub const BATTERY_ICON_X: u16 = 1;
pub const BATTERY_ICON_Y: u16 = 1;
pub const BATTERY_OUTLINE_COLOR: lcd_api::Color = lcd_api::Color::White;
pub const BATTERY_FILL_COLOR: lcd_api::Color = lcd_api::Color::Black;
pub const CHARGER_COLOR: lcd_api::Color = lcd_api::Color::Green;

//==============================================================================
// Public Functions
//==============================================================================
#[allow(dead_code)]
pub fn start_page(d: &mut info::DeviceInfo) {
	write_battery_level(d.battery_level as u8, d.battery_voltage, d.flags.charger_connected);
}

//==============================================================================
// Private Functions
//==============================================================================
fn write_battery_level(level: u8, voltage: u16, charging: bool) {
	// Draw outline
	lcd_api::fill_rectangle(BATTERY_ICON_X + 7, 29, BATTERY_ICON_Y, 15, BATTERY_OUTLINE_COLOR);

	// Fill in the middle of the battery
	let color = if charging { CHARGER_COLOR } else { BATTERY_FILL_COLOR };
	lcd_api::fill_rectangle(BATTERY_ICON_X + 9, 25, BATTERY_ICON_Y + 2, 11, color);

	// Print out battery level blocks
	for l in 0..level {
		let x = BATTERY_ICON_X + 10 + (l as u16 * 6);
		lcd_api::fill_rectangle(x, 5, BATTERY_ICON_Y + 3, 9, BATTERY_OUTLINE_COLOR);
	}

	// Write out the voltage value
	let mut divider: u16 = 1000;
	let color = if charging { CHARGER_COLOR } else { BATTERY_OUTLINE_COLOR };
	for d in 0..4 {
		let v: u8 = if d == 1 {
			0x2E
		}
		else {
			divider = divider / 10;
			((voltage / divider) % 10) as u8
		};
		let x = BATTERY_ICON_X + 8 + (d as u16 * 6);
		
		font::write_minimal_character((0x30 + v) as char, x, BATTERY_ICON_Y + 16, color, BATTERY_FILL_COLOR, 1)
	}

	// Print (or remove) the charger status icon
	let color = if charging { CHARGER_COLOR } else { BATTERY_FILL_COLOR };
	lcd_api::fill_rectangle(BATTERY_ICON_X, 7, BATTERY_ICON_Y, 15, color);
	if charging {
		lcd_api::fill_rectangle(BATTERY_ICON_X, 4, BATTERY_ICON_Y, 6, BATTERY_FILL_COLOR);
		lcd_api::fill_rectangle(BATTERY_ICON_X + 2, 2, BATTERY_ICON_Y + 9, 6, BATTERY_FILL_COLOR);
	}
	lcd_api::fill_rectangle(BATTERY_ICON_X + 5, 2, BATTERY_ICON_Y + 3, 9, BATTERY_OUTLINE_COLOR);
}

//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
#[allow(dead_code)]
pub fn task_handler(d: &mut info::DeviceInfo) {
	// Only change on swipe up - to close settings pulldown
	if d.change_flags.touch_event {
		if let Gesture::SlideDown = d.touch.gesture {
			d.app_page = page::AppPage::Home;
			page::change_page(d);
			return;
		}
	}

	// Return home on button press release
	if d.change_flags.button_press {
		if !d.flags.button_pressed {
			d.app_page = page::AppPage::Home;
			page::change_page(d);
			return;
		}
	}

	// Update reading when it chages
	if d.change_flags.battery_voltage {
		write_battery_level(d.battery_level as u8, d.battery_voltage, d.flags.charger_connected);
	}
}