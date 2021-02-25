//==============================================================================
// Notes
//==============================================================================
// drivers::button.rs
// Wrapper around the gpio pins for handling button presses

//==============================================================================
// Crates and Mods
//==============================================================================
use crate::config;
use crate::mcu::gpio;

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
	gpio::pin_setup(
		p, 
		config::PUSH_BUTTON_PIN,
		nrf52832_pac::p0::pin_cnf::DIR_A::INPUT,
		nrf52832_pac::p0::pin_cnf::PULL_A::PULLUP,
		gpio::PinState::PinLow
	);
}

//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
