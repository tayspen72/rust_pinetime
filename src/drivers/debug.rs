//==============================================================================
// Notes
//==============================================================================
// drivers::debug.rs

//==============================================================================
// Crates and Mods
//==============================================================================
// use crate::config;
// use crate::mcu::uart;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
use super::lcd::{lcd_api, font};

//==============================================================================
// Macros
//==============================================================================


//==============================================================================
// Variables
//==============================================================================
 static mut DEBUG_CURRENT_X: u16 = 9;
 static mut DEBUG_CURRENT_Y: u16 = 9;
 const DEBUG_SCALE: u16 = 2;
 const DEBUG_BACKGROUND: u16 = lcd_api::Color::Black as u16;
 const DEBUG_FOREGROUND: u16 = lcd_api::Color::White as u16;

//==============================================================================
// Implementations
//==============================================================================
#[allow(dead_code)]
pub fn init(p: &nrf52832_pac::Peripherals) {
	write_line(p, "**********");
	write_line(p, "*  Debug *");
	write_line(p, "**********");
}

pub fn write_line(p: &nrf52832_pac::Peripherals, string: &str) {
	let bytes = string.as_bytes();
	let len = bytes.len();

	for i in 0..len {
		write_character(p, bytes[i] as char);
	}

	unsafe {
		DEBUG_CURRENT_X = 9;
		let character_height: u16 = (font::MINIMAL_CHARACTER_HEIGHT + 1) * DEBUG_SCALE;

		DEBUG_CURRENT_Y += character_height;
		if (DEBUG_CURRENT_Y + character_height) > 240 {
			DEBUG_CURRENT_Y = 9;
		}
	}

}

fn write_character(p: &nrf52832_pac::Peripherals, c: char) {
	unsafe {
		font::write_minimal_character(p, c, DEBUG_CURRENT_X, DEBUG_CURRENT_Y, DEBUG_FOREGROUND, DEBUG_BACKGROUND, DEBUG_SCALE);
		DEBUG_CURRENT_X += (font::MINIMAL_CHARACTER_WIDTH + 1) * DEBUG_SCALE;
	}
}

//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
// pub fn task_handler() {
// 	uart::task_handler();
// }
