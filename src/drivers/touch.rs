//==============================================================================
// Notes
//==============================================================================
// drivers::lcd.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use crate::config;
use crate::mcu::i2c;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================


//==============================================================================
// Variables
//==============================================================================


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
fn configure() {
	//TODO: Fix this
	let p = unsafe { nrf52832_pac::Peripherals::steal() };
	i2c::read_byte(&p, config::TOUCH_I2C_ADDRESS, true);
}

//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
