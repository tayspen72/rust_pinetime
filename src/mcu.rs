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
//use cortex_m::Peripherals;


//=========================================================================
// Mods
//=========================================================================


//=========================================================================
// Types
//=========================================================================


//=========================================================================
// Variables
//=========================================================================
//static mut _PERIPHERALS: Option<nrf52832_pac::Peripherals> = None;


//=========================================================================
// Implementations
//=========================================================================
//TODO: Finish this
pub fn init(){
//    unsafe {_PERIPHERALS = Some(nrf52832_pac::Peripherals::take().unwrap()); }
//    let p =  nrf52832_pac::Peripherals::take().unwrap();
}

#[allow(dead_code)]
pub fn pin_setup(p: &nrf52832_pac::Peripherals, pin: u8, dir: PinDirection, state: PinState){
    //set pin direction
    match dir {
        PinDirection::PinInput => {
            p.P0.pin_cnf[pin as usize].write(|w| w.dir().input());
            p.P0.pin_cnf[pin as usize].write(|w| w.input().connect());
        },

        PinDirection::PinOutput => {
            p.P0.pin_cnf[pin as usize].write(|w| w.dir().output());
            unsafe{
                match state {
                    PinState::PinLow => p.P0.outclr.write(|w| w.bits(1 << pin)),
                    PinState::PinHigh => p.P0.outset.write(|w| w.bits(1 << pin)),
                    _ => ()
                };
            }
        }
    }
}

#[allow(dead_code)]
pub fn get_pin_state(p: &nrf52832_pac::Peripherals, pin: u8) -> PinState {
    let bits = p.P0.in_.read().bits() & (1 << pin);
    if bits.gt(&0) {
        PinState::PinHigh
    }
    else{
        PinState::PinLow
    }
}

#[allow(dead_code)]
pub fn set_pin_high(p: &nrf52832_pac::Peripherals, pin: u8) {
    unsafe { p.P0.outset.write(|w| w.bits(1 << pin)); }
}

#[allow(dead_code)]
pub fn set_pin_low(p: &nrf52832_pac::Peripherals, pin: u8) {
    unsafe { p.P0.outclr.write(|w| w.bits(1 << pin)); }
}

#[allow(dead_code)]
pub fn set_pin_state(p: &nrf52832_pac::Peripherals, pin: u8, state: PinState) {
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

