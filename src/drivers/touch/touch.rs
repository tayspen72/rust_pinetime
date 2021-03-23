//==============================================================================
// Notes
//==============================================================================
// drivers::touch.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use core::cell::Cell;
use cortex_m::interrupt::{free, Mutex};
use crate::config;
use crate::mcu::{input, i2c};
use super::cst816s;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================


//==============================================================================
// Variables
//==============================================================================
static TOUCH_EVENT: Mutex<Cell<cst816s::TouchEvent>> = Mutex::new(Cell::new( cst816s::TouchEvent {
	gesture: cst816s::Gesture::Unknown, event: cst816s::Event::Unknown, x: 0, y: 0, pressure: 0
}));

const TOUCH_INT_PIN_CONFIG: input::PinConfig = input::PinConfig {
	pin: config::TOUCH_INT_PIN,
	polarity: nrf52832_pac::gpiote::config::POLARITY_A::HITOLO,	//TODO: Need to verify this!
	pull: nrf52832_pac::p0::pin_cnf::PULL_A::PULLUP, // TODO: need to verify this
	callback: &touch_handler
};


const TOUCH_EVENT_READ_LEN: usize = 8;
static mut UNHANDLED_EVENTS: bool = false;

//==============================================================================
// Public Functions
//==============================================================================
pub fn init() {
	// Init the input interrupt
	input::init_pin(TOUCH_INT_PIN_CONFIG);
}

//==============================================================================
// Private Functions
//==============================================================================
fn get_event() {
	let mut buf: [u8; TOUCH_EVENT_READ_LEN] = [0; TOUCH_EVENT_READ_LEN];
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

fn touch_handler() {
	i2c::read_data(config::TOUCH_I2C_ADDRESS, true, TOUCH_EVENT_READ_LEN as u16);
	unsafe { UNHANDLED_EVENTS = true };
}

//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler() {
	unsafe { 
		if UNHANDLED_EVENTS {
			UNHANDLED_EVENTS = false;
			get_event();
			
			// Update device info
			// TODO: This
		}
	}
}