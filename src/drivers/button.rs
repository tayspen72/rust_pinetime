//==============================================================================
// Notes
//==============================================================================
// drivers::button.rs
// Wrapper around the gpio pins for handling button presses

//==============================================================================
// Crates and Mods
//==============================================================================
use crate::config;
use crate::drivers::lcd;
use crate::mcu::gpio;
use nrf52832_pac::p0::pin_cnf::DIR_A as DIR;
use nrf52832_pac::p0::pin_cnf::PULL_A as PULL;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================


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
	gpio::pin_setup(p, config::PUSH_BUTTON_IN_PIN, DIR::INPUT, gpio::PinState::PinLow, PULL::PULLUP);
	gpio::pin_setup(p, config::PUSH_BUTTON_OUT_PIN, DIR::OUTPUT, gpio::PinState::PinLow, PULL::DISABLED);
}

//==============================================================================
// Interrupt Handler
//==============================================================================
pub fn task_handler(p: &nrf52832_pac::Peripherals) {
	if let gpio::PinState::PinLow = gpio::get_pin_state(p, config::PUSH_BUTTON_IN_PIN) {
		lcd::lcd::set_backlight(p, lcd::lcd::BacklightBrightness::Brightness7);
	}
	else {
		lcd::lcd::set_backlight(p, lcd::lcd::BacklightBrightness::Brightness1);
	}
}

//==============================================================================
// Task Handler
//==============================================================================
