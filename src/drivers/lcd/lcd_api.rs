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
	Red			= 0x00F8,	// 1F, 00, 00
	Orange		= 0xE0FB,	// 1F, 1F, 00
	Yellow		= 0xE0FF,	// 1F, 3F, 00
	Green		= 0xE007,	// 00, 3F, 00
	Cyan		= 0xFF07,	// 00, 3F, 1F
	Blue		= 0x1F00,	// 00, 00, 1F
	Magenta		= 0x1FF8,	// 1F, 00, 1F
	White		= 0xFFFF,	// 1F, 3F, 1F
	GrayDark	= 0x0842,	// 08, 10, 08
	Gray		= 0xEF7B,	// 0F, 1F, 0F
	GrayLight	= 0x18C6,	// 18, 30, 18
}

//==============================================================================
// Variables
//==============================================================================


//==============================================================================
// Private Functions
//==============================================================================
pub fn init() {
	lcd::init();

	fill_background(Color::White);
	lcd::set_backlight(lcd::BacklightBrightness::Brightness7);
	
	write_splash();

	fill_background(Color::Black);
}

pub fn get_busy() -> bool {
	// For now, not using DMA, this library will never be busy
	false
}

#[allow(dead_code)]
pub fn fill_background(color: Color) {
	fill_rectangle(0, 240, 0, 240, color);
}

pub fn fill_rectangle(x: u16, width: u16, y: u16, height: u16, color: Color) {
	set_window(x, width, y, height);
	lcd::write_command(st7789::COMMAND::MEMORY_WRITE);
	lcd::write_block_solid(color as u16, (width*height) as u32);
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
	lcd::write_data( &[ x[1], x[0], x_end[1], x_end[0] ]);

	// Define the window row size
	lcd::write_command(st7789::COMMAND::ROW_ADDRESS);
	lcd::write_data( &[ y[1], y[0], y_end[1], y_end[0] ]);
}

fn write_splash() {
	set_window(39, 160, 59, 106);
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

