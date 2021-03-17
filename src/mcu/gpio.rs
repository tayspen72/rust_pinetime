//==============================================================================
// Notes
//==============================================================================
// mcu::gpio.rs
// Basic control over gpio pins

//==============================================================================
// Crates and Mods
//==============================================================================
use core::cell::RefCell;
use core::ops::DerefMut;
use cortex_m::interrupt::{free, Mutex};
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
// Variables
//==============================================================================
static GPIO_HANDLE: Mutex<RefCell<Option<nrf52832_pac::P0>>> = 
	Mutex::new(RefCell::new(None));

//==============================================================================
// Public Functions
//==============================================================================
pub fn init(p0: nrf52832_pac::P0) {
	free(|cs| GPIO_HANDLE.borrow(cs).replace(Some(p0)));
}

#[allow(dead_code)]
pub fn get_pin_state(pin: u8) -> PinState {
	let read = free(|cs| 
		if let Some(ref mut gpio) = GPIO_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			gpio.in_.read().bits()
		}
		else {
			0
		}
	);
	match read & (1 << pin) {
		0 => PinState::PinLow,
		_ => PinState::PinHigh
	}
}

#[allow(dead_code)]
pub fn pin_disable(pin: u8) {
	free(|cs| {
		if let Some(ref mut gpio) = GPIO_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			// Set as input and disconnect the buffer
			gpio.pin_cnf[pin as usize].modify(|_, w| w.dir().input());
			gpio.pin_cnf[pin as usize].modify(|_, w| w.input().disconnect());
		}
	});
}

#[allow(dead_code)]
pub fn pin_setup(pin: u8, dir: DIR, state: PinState, pull: PULL){
	free(|cs| {
		if let Some(ref mut gpio) = GPIO_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			// Set direction
			gpio.pin_cnf[pin as usize].modify(|_, w| w.dir().variant(dir));
			if let DIR::INPUT = dir {
				gpio.pin_cnf[pin as usize].modify(|_, w| w.input().connect());
			}
			else {
				gpio.pin_cnf[pin as usize].modify(|_, w| w.input().disconnect());
			}
			
			// Set pin pull
			gpio.pin_cnf[pin as usize].modify(|_, w| w.pull().variant(pull));

			// Set output state
			match state {
				PinState::PinLow => gpio.outclr.write(|w| unsafe {w.bits(1 << pin)}),
				PinState::PinHigh => gpio.outset.write(|w| unsafe {w.bits(1 << pin)})
			}
		}
	});
}

#[allow(dead_code)]
pub fn set_pin_state(pin: u8, state: PinState){
	match state {
		PinState::PinLow => {
			free(|cs| 
				if let Some(ref mut gpio) = GPIO_HANDLE.borrow(cs).borrow_mut().deref_mut() {
					gpio.outclr.write(|w| unsafe { w.bits(1 << pin) })
				}
			)
		},
		PinState::PinHigh => {
			free(|cs| 
				if let Some(ref mut gpio) = GPIO_HANDLE.borrow(cs).borrow_mut().deref_mut() {
					gpio.outset.write(|w| unsafe { w.bits(1 << pin) })
				}
			)
		}
	}
}

//==============================================================================
// Private Functions
//==============================================================================


//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
