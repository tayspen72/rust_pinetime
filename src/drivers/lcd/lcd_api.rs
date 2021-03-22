//==============================================================================
// Notes
//==============================================================================
// drivers::lcd.rs

//==============================================================================
// Crates and Mods
//==============================================================================
// use crate::config;
// use crate::mcu::spi;
use super::{images, lcd, st7789};

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
//TODO: Fix LCD colors
#[allow(dead_code)]
pub enum Color { // 5-6-5		R,  G,  B
	Black		= 0x0000,	//  0,  0,  0
	Red			= 0xF800,	// 1F, 00, 00
	Orange		= 0xFBE0,	// 1F, 1F, 00
	Yellow		= 0xFFE0,	// 1F, 3F, 00
	Green		= 0x07E0,	// 00, 3F, 00
	Cyan		= 0x07FF,	// 00, 3F, 1F
	Blue		= 0x001F,	// 00, 00, 1F
	Magenta		= 0xF81F,	// 1F, 00, 1F
	White		= 0xFFFF,	// 1F, 3F, 1F

	GrayDark	= 0x4208,	// 08, 10, 08
	Gray		= 0x7BEF,	// 0F, 1F, 0F
	GrayLight	= 0xC618,	// 18, 30, 18
}

//==============================================================================
// Variables
//==============================================================================


//==============================================================================
// Private Functions
//==============================================================================
pub fn init() {
	lcd::init();
	lcd::set_backlight(lcd::BacklightBrightness::Brightness7);

	fill_rectangle(0, 79, 0, 79, Color::Black as u16);
	fill_rectangle(81, 78, 0, 79, Color::Red as u16);
	fill_rectangle(161, 79, 0, 79, Color::Orange as u16);
	fill_rectangle(0, 79, 81, 78, Color::Yellow as u16);
	fill_rectangle(81, 78, 81, 78, Color::Green as u16);
	fill_rectangle(161, 79, 81, 78, Color::Cyan as u16);
	fill_rectangle(0, 79, 161, 79, Color::Blue as u16);
	fill_rectangle(81, 78, 161, 79, Color::Magenta as u16);
	fill_rectangle(161, 79, 161, 79, Color::White as u16);

	write_image();
	write_image_dma();
}

pub fn get_busy() -> bool {
	// For now, not using DMA, this library will never be busy
	false
}

#[allow(dead_code)]
pub fn fill_background(color: u16) {
	fill_rectangle(0, 240, 0, 240, color);
}

pub fn fill_rectangle(x: u16, width: u16, y: u16, height: u16, color: u16) {
	set_window(x, width, y, height);
	lcd::write_command(st7789::COMMAND::MEMORY_WRITE);
	lcd::write_block_solid(color, (width*height) as u32);
}

pub fn set_window(x: u16, width: u16, y: u16, height: u16) {
	let x_end = x + width - 1;
	let y_end = y + height - 1;

	// TODO: Check that this endian conversion is correct
	let x = x.to_le_bytes();
	let x_end = x_end.to_le_bytes();
	let y = y.to_le_bytes();
	let y_end = y_end.to_le_bytes();

	// Define the window column size
	lcd::write_command(st7789::COMMAND::COLUMN_ADDRESS);
	lcd::write_data( &[ x[0], x[1], x_end[0], x_end[1] ]);

	// Define the window row size
	lcd::write_command(st7789::COMMAND::ROW_ADDRESS);
	lcd::write_data( &[ y[0], y[1], y_end[0], y_end[1] ]);
}

fn write_image() {
	set_window(79, 80, 79, 53);
	lcd::write_command(st7789::COMMAND::MEMORY_WRITE);
	lcd::write_data(&images::RUSTACEAN);
}

fn write_image_dma() {
	set_window(79, 50, 159, 53);
	lcd::write_command(st7789::COMMAND::MEMORY_WRITE);
	lcd::write_block(&images::RUSTACEAN);
}

//==============================================================================
// Public Functions
//==============================================================================


//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================

