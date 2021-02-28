//==============================================================================
// Notes
//==============================================================================
// mcu::gpio.rs
// Basic control over gpio pins

//==============================================================================
// Crates and Mods
//==============================================================================
use nrf52832_pac;

use nrf52832_pac::p0::pin_cnf::DIR_A as DIR;
use nrf52832_pac::p0::pin_cnf::PULL_A as PULL;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub enum PinDirection{
	Input,
	Output
}

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub enum PinPull{
	PullUp,
	PullDown,
	PullDisabled
}

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub enum PinState{
	PinLow,
	PinHigh
}

//==============================================================================
// Macros
//==============================================================================


//==============================================================================
// Variables
//==============================================================================


//==============================================================================
// Implementations
//==============================================================================
#[allow(dead_code)]
pub fn get_pin_state(p: &nrf52832_pac::Peripherals, pin: u8) -> PinState {
	match p.P0.in_.read().bits() & (1 << pin) {
		0 => PinState::PinLow,
		_ => PinState::PinHigh
	}
}

#[allow(dead_code)]
pub fn pin_disable(p: &nrf52832_pac::Peripherals, pin: u8) {
	// Set as input and disconnect the buffer
	p.P0.pin_cnf[pin as usize].modify(|_, w| w.dir().input());
	p.P0.pin_cnf[pin as usize].modify(|_, w| w.input().disconnect());
}

#[allow(dead_code)]
pub fn pin_setup(p: &nrf52832_pac::Peripherals, pin: u8, dir: DIR, state: PinState, pull: PULL){
		// Set direction
	p.P0.pin_cnf[pin as usize].modify(|_, w| w.dir().variant(dir));
	if let DIR::INPUT = dir {
		p.P0.pin_cnf[pin as usize].modify(|_, w| w.input().connect());
	}
	else {
		p.P0.pin_cnf[pin as usize].modify(|_, w| w.input().disconnect());
	}
	
	// Set pin pull
	p.P0.pin_cnf[pin as usize].modify(|_, w| w.pull().variant(pull));

	// Set output state
	match state {
		PinState::PinLow => p.P0.outclr.write(|w| unsafe {w.bits(1 << pin)}),
		PinState::PinHigh => p.P0.outset.write(|w| unsafe {w.bits(1 << pin)})
	}
}

#[allow(dead_code)]
pub fn set_pin_state(p: &nrf52832_pac::Peripherals, pin: u8, state: PinState){
	match state {
		PinState::PinLow => p.P0.outclr.write(|w| unsafe {w.bits(1 << pin)}),
		PinState::PinHigh => p.P0.outset.write(|w| unsafe {w.bits(1 << pin)})
	}
}

//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
