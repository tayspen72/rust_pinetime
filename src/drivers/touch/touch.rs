//==============================================================================
// Notes
//==============================================================================
// drivers::lcd.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use core::cell::Cell;
use cortex_m::interrupt::{free, Mutex};
use crate::config;
use crate::mcu::i2c;
use super::cst8165;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
pub enum GestureId {
	None,
	SlideUp,
	SlideDown,
	SlideLeft,
	SlideRight,
	SinglePress,
	DoublePress,
	LongPress
}

pub struct TouchEvent{
	pub x: u8,
	pub y: u8,
	pub gesture: GestureId,
	pub pressure: u8 
}

//==============================================================================
// Variables
//==============================================================================
static TOUCH_EVENT: Mutex<Cell<TouchEvent>> = Mutex::new(Cell::new( TouchEvent {
	x: 0, y: 0, gesture: GestureId::None, pressure: 0
}));

//==============================================================================
// Public Functions
//==============================================================================
#[allow(dead_code)]
pub fn init() {
	configure();
}

//==============================================================================
// Private Functions
//==============================================================================
fn check_connected() -> bool {
	let id = read_register(0x00).unwrap();
	if id == cst8165::WHO_AM_I_VALUE {
		true
	}
	else {
		false
	}
}

fn configure() {
	if !check_connected() {
		return;
	}
}

fn get_event() {
	let mut buf: [u8; 63] = [0; 63];
	i2c::read_data(config::TOUCH_I2C_ADDRESS, true, 63);
	for i in 0..buf.len() {
		buf[i] = i2c::pop_byte();
	}

	//TODO: Build this event from the register
	let mut event: TouchEvent;

	
}

fn read_register(reg: u8) -> Option<u8> {
	if i2c::write_byte(config::TOUCH_I2C_ADDRESS, reg, true, false).unwrap() {
		i2c::read_byte(config::TOUCH_I2C_ADDRESS, true);
		return Some(i2c::pop_byte());
	}

	None
}

#[allow(dead_code)]
fn write_register(reg: u8, data: &[u8]) -> Option<bool> {
	if i2c::write_byte(config::TOUCH_I2C_ADDRESS, reg, true, false).unwrap() {
		return i2c::write_data(config::TOUCH_I2C_ADDRESS, data, false, true);
	}

	None
}

//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
