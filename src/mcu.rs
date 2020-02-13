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
use core::ops::BitAnd;


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
pub fn init(){
    ();
}

#[allow(dead_code)]
pub fn pin_setup(pin: u8, dir: PinDirection, state: PinState){
    let p =  nrf52832_pac::Peripherals::take().unwrap();

    //set pin direction
    match dir {
        PinDirection::PinInput => {
            p.P0.pin_cnf[pin as usize].write(|w| w.dir().input());
            p.P0.pin_cnf[pin as usize].write(|w| w.input().connect());
        },

        PinDirection::PinOutput => {
            p.P0.pin_cnf[pin as usize].write(|w| w.dir().output());
            unsafe {
                match state {
                    PinState::PinLow => p.P0.outclr.write(|w| w.bits(1 << pin)),
                    PinState::PinHigh => p.P0.outset.write(|w| w.bits(1 << pin)),
                    _ => ()
                };
            }
        }
    };
}

#[allow(dead_code)]
pub fn get_pin_state(pin: u8) -> PinState {
    let p =  nrf52832_pac::Peripherals::take().unwrap();
    if p.P0.in_.read().bits().bitand(1 << pin).gt(&0) {
        PinState::PinHigh
    }
    else{
        PinState::PinLow
    }
}

#[allow(dead_code)]
pub fn set_pin_high(pin: u8) {
    let p =  nrf52832_pac::Peripherals::take().unwrap();
    unsafe { p.P0.outset.write(|w| w.bits(1 << pin)); }
}

#[allow(dead_code)]
pub fn set_pin_low(pin: u8) {
    let p =  nrf52832_pac::Peripherals::take().unwrap();
    unsafe { p.P0.outclr.write(|w| w.bits(1 << pin)); }
}

#[allow(dead_code)]
pub fn set_pin_state(pin: u8, state: PinState) {
    let p =  nrf52832_pac::Peripherals::take().unwrap();

    unsafe {
        match state{
            PinState::PinLow => p.P0.outclr.write(|w| w.bits(1 << pin)),
            PinState::PinHigh => p.P0.outset.write(|w| w.bits(1 << pin)),
            _ => ()
        };
    };
}


//=========================================================================
// TaskHandler
//=========================================================================


//=========================================================================
// Interrupt
//=========================================================================

