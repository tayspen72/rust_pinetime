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
use super::{app, debug};

//==============================================================================
// Enums, Structs, and Types
//==============================================================================


//==============================================================================
// Variables
//==============================================================================
const BUTTON_PRESS_CONFIG: input::PinConfig = input::PinConfig {
	pin: config::PUSH_BUTTON_IN_PIN,
	polarity: nrf52832_pac::gpiote::config::POLARITY_A::TOGGLE,
	pull: nrf52832_pac::p0::pin_cnf::PULL_A::PULLUP,
	callback: &press_handler,
	real_time_callback: false
};

const PRESS_QUEUE_LENGTH: usize = 8;

static mut UNHANDLED_PRESSES: u8 = 0;
static mut PRESS_QUEUE: [bool; PRESS_QUEUE_LENGTH] = [false; PRESS_QUEUE_LENGTH];
static mut HEAD: usize = 0;
static mut TAIL: usize = 0;

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
	unsafe { 
		UNHANDLED_PRESSES = UNHANDLED_PRESSES + 1;
		PRESS_QUEUE[TAIL] = match gpio::get_pin_state(config::PUSH_BUTTON_IN_PIN) {
			gpio::PinState::PinHigh => false,
			gpio::PinState::PinLow => true,
		};
		TAIL = if (TAIL + 1) < PRESS_QUEUE_LENGTH { TAIL + 1 } else { 0 };
	};
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
			d.change_flags.button_press = true;
			d.flags.button_pressed = PRESS_QUEUE[HEAD];
			HEAD = if (HEAD + 1) < PRESS_QUEUE_LENGTH { HEAD + 1 } else { 0 };
			UNHANDLED_PRESSES = UNHANDLED_PRESSES - 1;

			if debug::is_enabled() {
				if d.flags.button_pressed {
					debug::push_log("Button pressed!");
				}
				else {
					debug::push_log("Button released!");
				}
			}
		}
	}
}