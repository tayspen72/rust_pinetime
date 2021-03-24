//==============================================================================
// Notes
//==============================================================================
// drivers::touch.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use crate::config;
use crate::drivers::debug;
use crate::mcu::{gpio, input, i2c};
use super::cst816s;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================


//==============================================================================
// Variables
//==============================================================================
const TOUCH_INT_PIN_CONFIG: input::PinConfig = input::PinConfig {
	pin: config::TOUCH_INT_PIN,
	polarity: nrf52832_pac::gpiote::config::POLARITY_A::HITOLO,
	pull: nrf52832_pac::p0::pin_cnf::PULL_A::DISABLED,
	callback: &touch_handler,
	real_time_callback: true
};

const TOUCH_EVENT_READ_LEN: usize = 8;
static mut UNHANDLED_EVENTS: bool = false;

//==============================================================================
// Public Functions
//==============================================================================
pub fn init() {
	// Init the input interrupt
	gpio::pin_setup(
		config::TOUCH_RESET_PIN,
		nrf52832_pac::p0::pin_cnf::DIR_A::OUTPUT,
		gpio::PinState::PinHigh,
		nrf52832_pac::p0::pin_cnf::PULL_A::DISABLED
	);
	
	input::init_pin(TOUCH_INT_PIN_CONFIG);
}

//==============================================================================
// Private Functions
//==============================================================================
fn get_event() -> cst816s::TouchEvent {
	let mut buf: [u8; TOUCH_EVENT_READ_LEN] = [0; TOUCH_EVENT_READ_LEN];
	for i in 0..buf.len() {
		buf[i] = i2c::pop_byte();
	}

	let touch: cst816s::TouchEvent = cst816s::TouchEvent {
		gesture: cst816s::get_gesture(buf[3]),
		event: cst816s::get_event(buf[3]),
		x: cst816s::get_coordinate(buf[3], buf[4]),
		y: cst816s::get_coordinate(buf[5], buf[6]),
		pressure: cst816s::get_pressure(buf[7])
	};

	debug::push_log_number("event: ", &(touch.event as u32));
	debug::push_log_number("x: ", &(touch.x as u32));
	debug::push_log_number("y: ", &(touch.y as u32));

	touch
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