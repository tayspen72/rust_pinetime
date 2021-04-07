//==============================================================================
// Notes
//==============================================================================
// app::page::home.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use crate::drivers::lcd::{font, lcd_api};
use crate::mcu::timer;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================


//==============================================================================
// Variables
//==============================================================================


//==============================================================================
// Public Functions
//==============================================================================
#[allow(dead_code)]
pub fn print_page(){
	// Show the startup sequence
	lcd_api::fill_background(lcd_api::Color::White);
	lcd_api::set_backlight(lcd_api::BacklightBrightness::Brightness4);
	font::write_minimal_line(b"Tsp Designs", 174, 231, lcd_api::Color::Navy, lcd_api::Color::White, 1);
	lcd_api::write_splash();
	font::write_minimal_line(b"Powered by", 60, 179, lcd_api::Color::Rust, lcd_api::Color::White, 2);
	font::write_minimal_line(b"RUST", 85, 200, lcd_api::Color::Rust, lcd_api::Color::White, 3);
	timer::delay(2000);

	// lcd_api::set_backlight(lcd_api::BacklightBrightness::Brightness0);
	// lcd_api::fill_background(lcd_api::Color::Black);

	// lcd_api::set_backlight(lcd_api::BacklightBrightness::Brightness4);
	// lcd_api::fill_rectangle(0, 79, 0, 79, lcd_api::Color::Black);
	// lcd_api::fill_rectangle(81, 78, 0, 79, lcd_api::Color::Red);
	// lcd_api::fill_rectangle(161, 79, 0, 79, lcd_api::Color::Orange);
	// lcd_api::fill_rectangle(0, 79, 81, 78, lcd_api::Color::Yellow);
	// lcd_api::fill_rectangle(81, 78, 81, 78, lcd_api::Color::Green);
	// lcd_api::fill_rectangle(161, 79, 81, 78, lcd_api::Color::Cyan);
	// lcd_api::fill_rectangle(0, 79, 161, 79, lcd_api::Color::Blue);
	// lcd_api::fill_rectangle(81, 78, 161, 79, lcd_api::Color::Magenta);
	// lcd_api::fill_rectangle(161, 79, 161, 79, lcd_api::Color::White);
	timer::delay(2000);
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
