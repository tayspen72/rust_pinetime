//==============================================================================
// Notes
//==============================================================================
// drivers::touch::cst816s.rs
// Register definitions for the CST8165 Touch Sensor 

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Gesture {
	None			= 0x0,
	SlideDown		= 0x1,
	SlideUp			= 0x2,
	SlideLeft		= 0x3,
	SlideRight		= 0x4,
	SinglePress		= 0x5,
	DoublePress		= 0xB,
	LongPress		= 0xC,
	Unknown			= 0xF
}
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Event {
	Down			= 0x0,
	Up				= 0x1,
	Contact			= 0x2,
	Unknown			= 0x3
}

#[derive(Clone, Copy)]
pub struct TouchEvent{
	pub gesture: Gesture,
	pub event: Event,
	pub x: u16,
	pub y: u16,
	pub pressure: u8 
}

//==============================================================================
// Variables
//==============================================================================
#[allow(dead_code)]
pub const I2C_ADDRESS: u8 				= 0x15;

//==============================================================================
// Public Functions
//==============================================================================
pub fn get_event(raw: u8) -> Event {
	match (raw & 0xC0) >> 6 {
		0 => Event::Down,
		1 => Event::Up,
		2 => Event::Contact,
		_ => Event::Unknown
	}
}

pub fn get_gesture(raw: u8) -> Gesture {
	match raw {
		0x0 => Gesture::None,
		0x1 => Gesture::SlideDown,
		0x2 => Gesture::SlideUp,
		0x3 => Gesture::SlideLeft,
		0x4 => Gesture::SlideRight,
		0x5 => Gesture::SinglePress,
		0xB => Gesture::DoublePress,
		0xC => Gesture::LongPress,
		_ => Gesture::Unknown
	}
}

pub fn get_pressure(raw: u8) -> u8 {
	raw
}

pub fn get_coordinate(raw_msb: u8, raw_lsb: u8) -> u16 {
	let mut c: u16 = (((raw_msb & 0x0F) as u16) << 8) as u16;
	c = c | (raw_lsb as u16);
	c
}

