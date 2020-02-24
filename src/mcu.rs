/*
 * mcu.rs
 *
 * Created:  12 Feb 2020
 * Author: T. Spencer
 */

//=========================================================================
// Notes
//=========================================================================
/* nRF52832 */


//=========================================================================
// Definitions
//=========================================================================
#[allow(dead_code)]
pub enum PinDirection {
    PinInput = 1,
    PinOutput = 0
}

#[allow(dead_code)]
pub enum PinState {
    PinLow,
    PinHigh,
    NA
}


//=========================================================================
// Crates
//=========================================================================
use nrf52832_pac;
use cortex_m;


//=========================================================================
// Mods
//=========================================================================


//=========================================================================
// Types
//=========================================================================


//=========================================================================
// Variables
//=========================================================================


//=========================================================================
// Implementations
//=========================================================================
//TODO: Finish this
pub fn init(_cp: &cortex_m::Peripherals, _p: &nrf52832_pac::Peripherals) {

}

pub fn get_core_peripherals() -> cortex_m::Peripherals {
    cortex_m::Peripherals::take().unwrap()
}

pub fn get_peripherals() -> nrf52832_pac::Peripherals {
    nrf52832_pac::Peripherals::take().unwrap()
}

pub fn pin_setup(p: &nrf52832_pac::Peripherals, pin: u8, dir: PinDirection, state: PinState){
    //set pin direction
    match dir {
        PinDirection::PinInput => {
            p.P0.pin_cnf[pin as usize].modify(|_, w| w.dir().input());
            p.P0.pin_cnf[pin as usize].modify(|_, w| w.input().connect());
        },

        PinDirection::PinOutput => {
            p.P0.pin_cnf[pin as usize].modify(|_, w| w.dir().output());
            unsafe {
                match state {
                    PinState::PinLow => p.P0.outclr.modify(|_, w| w.bits(1 << pin)),
                    PinState::PinHigh => p.P0.outset.modify(|_, w| w.bits(1 << pin)),
                    _ => ()
                };
            }
        }
    }
}

pub fn get_pin_state(p: &nrf52832_pac::Peripherals, pin: u8) -> PinState {
    let bits = p.P0.in_.read().bits() & (1 << pin);

    if bits.gt(&0) {
        PinState::PinHigh
    } else {
        PinState::PinLow
    }
}

pub fn set_pin_high(p: &nrf52832_pac::Peripherals, pin: u8) {
    unsafe{ p.P0.outset.write(|w| w.bits(1 << pin)); }
}

pub fn set_pin_low(p: &nrf52832_pac::Peripherals, pin: u8) {
    unsafe{ p.P0.outclr.write(|w| w.bits(1 << pin)); }
}

#[allow(dead_code)]
pub fn set_pin_state(p: &nrf52832_pac::Peripherals, pin: u8, state: PinState) {
    unsafe{
        match state{
            PinState::PinLow => p.P0.outclr.write(|w| w.bits(1 << pin)),
            PinState::PinHigh => p.P0.outset.write(|w| w.bits(1 << pin)),
            _ => ()
        };
    }
}


//=========================================================================
// TaskHandler
//=========================================================================


//=========================================================================
// Interrupt
//=========================================================================
