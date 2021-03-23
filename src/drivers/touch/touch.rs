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
use super::cst816s;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================


//==============================================================================
// Variables
//==============================================================================
#[allow(dead_code)]
static TOUCH_EVENT: Mutex<Cell<cst816s::TouchEvent>> = Mutex::new(Cell::new( cst816s::TouchEvent {
	gesture: cst816s::Gesture::Unknown, event: cst816s::Event::Unknown, x: 0, y: 0, pressure: 0
}));

//==============================================================================
// Public Functions
//==============================================================================
#[allow(dead_code)]
pub fn init() {

}

//==============================================================================
// Private Functions
//==============================================================================
#[allow(dead_code)]
fn get_event() {
	let mut buf: [u8; 63] = [0; 63];
	for i in 0..buf.len() {
		buf[i] = i2c::pop_byte();
	}
	
	free(|cs| {
		let mut touch: cst816s::TouchEvent = TOUCH_EVENT.borrow(cs).get();
		touch.gesture = cst816s::get_gesture(buf[3]);
		touch.event = cst816s::get_event(buf[3]);
		touch.x = cst816s::get_coordinate(buf[3], buf[4]);
		touch.y = cst816s::get_coordinate(buf[5], buf[6]);
		touch.pressure = cst816s::get_pressure(buf[7]);

		TOUCH_EVENT.borrow(cs).set(touch);
	});
}

#[allow(dead_code)]
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
// pub fn task_handler() {
	// if pending_events {
	// 	get_event();
		
	// 	// Update device info
	// 	black magic
	// }
// }