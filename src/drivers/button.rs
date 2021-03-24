//==============================================================================
// Notes
//==============================================================================
// drivers::button.rs
// Wrapper around the gpio pins for handling button presses

//==============================================================================
// Crates and Mods
//==============================================================================
use crate::config;
use crate::mcu::{gpio, input};
use nrf52832_pac::p0::pin_cnf::DIR_A as DIR;
use nrf52832_pac::p0::pin_cnf::PULL_A as PULL;
use super::app;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================


//==============================================================================
// Variables
//==============================================================================
const BUTTON_PRESS_CONFIG: input::PinConfig = input::PinConfig {
	pin: config::PUSH_BUTTON_IN_PIN,
	polarity: nrf52832_pac::gpiote::config::POLARITY_A::HITOLO,
	pull: nrf52832_pac::p0::pin_cnf::PULL_A::PULLUP,
	callback: &press_handler,
	real_time_callback: false
};

static mut UNHANDLED_PRESSES: u8 = 0;

//==============================================================================
// Public Functions
//==============================================================================
pub fn init() {
	input::init_pin(BUTTON_PRESS_CONFIG);
	gpio::pin_setup(config::PUSH_BUTTON_OUT_PIN, DIR::OUTPUT, gpio::PinState::PinLow, PULL::DISABLED);
}

//==============================================================================
// Private Functions
//==============================================================================
#[allow(dead_code)]
fn press_handler() {
	unsafe { UNHANDLED_PRESSES = UNHANDLED_PRESSES + 1 };
}

//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler(d: &mut app::DeviceInfo) {
	unsafe {
		if UNHANDLED_PRESSES > 0 {
			d.flags.button_press = true;
			d.button_press_count = UNHANDLED_PRESSES;
			UNHANDLED_PRESSES = 0;
		}
	}
}