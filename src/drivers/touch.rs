//==============================================================================
// Notes
//==============================================================================
// drivers::touch.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use crate::config;
use crate::drivers::{app, debug};
use crate::mcu::{gpio, input, i2c};

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
pub fn get_coordinate(raw_msb: u8, raw_lsb: u8) -> u16 {
	let mut c: u16 = (((raw_msb & 0x0F) as u16) << 8) as u16;
	c = c | (raw_lsb as u16);
	c
}

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

fn read_event() -> TouchEvent {
	let mut buf: [u8; TOUCH_EVENT_READ_LEN] = [0; TOUCH_EVENT_READ_LEN];
	for i in 0..buf.len() {
		buf[i] = i2c::pop_byte();
	}

	let touch: TouchEvent = TouchEvent {
		gesture: get_gesture(buf[3]),
		event: get_event(buf[3]),
		x: get_coordinate(buf[3], buf[4]),
		y: get_coordinate(buf[5], buf[6]),
		pressure: get_pressure(buf[7])
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
pub fn task_handler(d: &mut app::DeviceInfo) {
	if d.flags.touch_event {
		d.flags.touch_event = false;
	}

	unsafe { 
		if UNHANDLED_EVENTS {
			UNHANDLED_EVENTS = false;
			d.touch = read_event();
			d.flags.touch_event = true;
		}
	}
}