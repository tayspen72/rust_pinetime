//==============================================================================
// Notes
//==============================================================================
// drivers::lcd.rs

//==============================================================================
// Crates and Mods
//==============================================================================
// use crate::config;
// use crate::mcu::spi;
use super::{lcd, st7789};

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[allow(dead_code)]
pub enum Color { // 5-6-5		R,  G,  B
	Black		= 0x0000,	//  0,  0,  0
	Red			= 0xF800,	// 1F, 00, 00
	Orange		= 0xFFE0,	// 1F, 1F, 00
	Yellow		= 0x07FF,	// 1F, 3F, 00
	Green		= 0x07E0,	// 00, 3F, 00
	Blue		= 0x001F,	// 00, 00, 1F
	Purple		= 0xF81F,	// 1F, 00, 1F
	White		= 0x0004,	// 1F, 3F, 1F

	GrayDark	= 0x0001,	// 08, 10, 08
	Gray		= 0x0002,	// 0F, 1F, 0F
	GrayLight	= 0x0003,	// 18, 30, 18

	YellowGreen	= 0x0008,	// 0F, 3F, 00
	TealGreen	= 0x000a,	// 00, 3F, 0F
	Teal		= 0x000b,	// 00, 3F, 1F
	TealBlue	= 0x000c,	// 00, 1F, 1F
	Navy		= 0x000e,	// 00, 00, 0F
	Magenta		= 0x000f,	// 0F, 00, 1F
	Pink		= 0x0020,	// 1F, 00, 0F
}

//==============================================================================
// Macros
//==============================================================================


//==============================================================================
// Variables
//==============================================================================


//==============================================================================
// Implementations
//==============================================================================
pub fn init(p: &nrf52832_pac::Peripherals) {
	lcd::init(p);

	lcd::set_backlight(p, lcd::BacklightBrightness::Brightness7);
	fill_background(p, Color::Red as u16);
	fill_background(p, Color::Orange as u16);
	fill_background(p, Color::Yellow as u16);
	fill_background(p, Color::Green as u16);
	fill_background(p, Color::Blue as u16);
	fill_background(p, Color::Purple as u16);
	fill_background(p, Color::Black as u16);
}

pub fn fill_background(p: &nrf52832_pac::Peripherals, color: u16) {
	set_window(p, 0, 239, 0, 239);
	lcd::write_command(p, st7789::COMMAND_A::MEMORY_WRITE);
	for _ in 0..57600 {
		lcd::write_data(p, &[ ((color & 0xFF00) >> 8)as u8, (color & 0xFF) as u8 ]);
	}
}

pub fn set_window(p: &nrf52832_pac::Peripherals, x_start: u16, x_end: u16, y_start: u16, y_end: u16) {
	// Define the window column size
	lcd::write_command(p, st7789::COMMAND_A::COLUMN_ADDRESS);
	lcd::write_data(p, &[ 
		((x_start & 0xFF00) >> 8) as u8, (x_start & 0x00FF) as u8,
		((x_end & 0xFF00) >> 8) as u8, (x_end & 0x00FF) as u8,
	]);

	// Define the window row size
	lcd::write_command(p, st7789::COMMAND_A::ROW_ADDRESS);
	lcd::write_data(p, &[ 
		((y_start & 0xFF00) >> 8) as u8, (y_start & 0x00FF) as u8,
		((y_end & 0xFF00) >> 8) as u8, (y_end & 0x00FF) as u8,
	]);
}

//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler() {

}
