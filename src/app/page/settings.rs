//==============================================================================
// Notes
//==============================================================================
// app::page::settings.rs

// TODO: Make the settings page show 9 icons - each 60x40 pixels
// TODO: Handle touch event when pressing the icon - eg brightness adjust

//==============================================================================
// Crates and Mods
//==============================================================================
use crate::app::{info, page};
use crate::drivers::lcd::{font, lcd_api};
use crate::drivers::touch::Gesture;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
struct BatteryIcon {
	x: u16,
	y: u16,
	low_color: lcd_api::Color,
	charging_color: lcd_api::Color,
	outline_color: lcd_api::Color,
}

//==============================================================================
// Variables
//==============================================================================
// Battery and Charger
const BATTERY: BatteryIcon = BatteryIcon {
	x: 10,
	y: 40,
	low_color: lcd_api::Color::Red,
	charging_color: lcd_api::Color::Green,
	outline_color: lcd_api::Color::White
};

const BACKGROUND_COLOR: lcd_api::Color = lcd_api::Color::Black;

//==============================================================================
// Public Functions
//==============================================================================
#[allow(dead_code)]
pub fn start_page(d: &mut info::DeviceInfo) {
	font::write_minimal_line(
		b"Settings",
		56,
		3,
		lcd_api::Color::White,
		lcd_api::Color::Black,
		3
	);
	write_all_icons(d);
	
}

//==============================================================================
// Private Functions
//==============================================================================
pub fn write_all_icons(d: &mut info::DeviceInfo) {
	write_battery_icon(BATTERY.x, BATTERY.y, d);
}

fn write_battery_icon(x: u16, y: u16, d: &mut info::DeviceInfo) {
	// Draw outline
	let level = d.battery_level as u8;
	let voltage = d.battery_voltage;
	let charging = d.flags.charger_connected;

	let outline = if level == 0 { BATTERY.low_color } else { BATTERY.outline_color };
	let fill = if charging { BATTERY.charging_color } else { BACKGROUND_COLOR };

	// Draw outline
	lcd_api::fill_rectangle(x + 12, 48, y, 5, outline);
	lcd_api::fill_rectangle(x + 12, 5, y + 5, 24, outline);
	lcd_api::fill_rectangle(x + 55, 5, y + 5, 24, outline);
	lcd_api::fill_rectangle(x + 12, 48, y + 29, 5, outline);
	if charging {
		lcd_api::fill_rectangle(x, 4, y + 12, 10, BATTERY.charging_color);
		lcd_api::fill_rectangle(x + 4, 8, y + 2, 30, BATTERY.charging_color);
	}
	else {
		lcd_api::fill_rectangle(x + 4, 8, y + 2, 4, BACKGROUND_COLOR);
		lcd_api::fill_rectangle(x, 8, y + 6, 22, BACKGROUND_COLOR);
		lcd_api::fill_rectangle(x + 4, 8, y + 28, 4, BACKGROUND_COLOR);
	}
	lcd_api::fill_rectangle(x + 8, 4, y + 6, 22, outline);

	// Fill battery
	lcd_api::fill_rectangle(x + 17, 38, y + 5, 24, fill);

	// Print out battery level blocks
	if level > 0 {
		for lev in 0..(level - 1) {
			lcd_api::fill_rectangle(
				x + 19 + (lev as u16 * 9), 
				7,
				y + 7, 
				20,
				outline
			);
		}
	}

	// Write out the voltage value
	let outline = if charging { BATTERY.charging_color } else { BATTERY.outline_color };
	let string: [u8; 5] = [
		(0x30 + (voltage / 100) % 10) as u8,
		0x2E,
		(0x30 + (voltage / 10) % 10) as u8,
		(0x30 + (voltage % 10)) as u8,
		'v' as u8
	];
	font::write_minimal_line(
		&string, 
		x + 5, 
		y + 36, 
		outline, 
		BACKGROUND_COLOR,
		2
	);
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
		if let Gesture::SlideUp = d.touch.gesture {
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
	if d.change_flags.battery_voltage  || d.change_flags.charger_state {
		write_battery_icon(BATTERY.x, BATTERY.y, d);
	}
}