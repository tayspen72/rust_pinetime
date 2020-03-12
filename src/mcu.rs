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

#[allow(dead_code)]
pub enum PinPull {
    None,
    PullDown,
    PullUp
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
pub fn init() {
}

#[allow(dead_code)]
pub fn get_core_peripherals() -> cortex_m::Peripherals {
    cortex_m::Peripherals::take().unwrap()
}

#[allow(dead_code)]
pub fn get_peripherals() -> nrf52832_pac::Peripherals {
    nrf52832_pac::Peripherals::take().unwrap()
}

#[allow(dead_code)]
pub fn pin_setup(pin: u8, dir: PinDirection, state: PinState, pull: PinPull){

    let p = get_peripherals();
    //set pin direction
    match dir {
        PinDirection::PinInput => {
            p.P0.pin_cnf[pin as usize].modify(|_, w| w.dir().input());
            p.P0.pin_cnf[pin as usize].modify(|_, w| w.input().connect());
            match pull {
                PinPull::None => p.P0.pin_cnf[pin as usize].modify(|_, w| w.pull().disabled()),
                PinPull::PullDown => p.P0.pin_cnf[pin as usize].modify(|_, w| w.pull().pulldown()),
                PinPull::PullUp => p.P0.pin_cnf[pin as usize].modify(|_, w| w.pull().pullup())
            };
        },

        PinDirection::PinOutput => {
            p.P0.pin_cnf[pin as usize].modify(|_, w| w.dir().output());
            p.P0.pin_cnf[pin as usize].modify(|_, w| w.input().disconnect());
            p.P0.pin_cnf[pin as usize].modify(|_, w| w.drive().s0s1());
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

#[allow(dead_code)]
pub fn get_pin_state(pin: u8) -> PinState {
    let p = get_peripherals();

    let bits = p.P0.in_.read().bits() & (1 << pin);

    if bits.gt(&0) {
        PinState::PinHigh
    } else {
        PinState::PinLow
    }
}

#[allow(dead_code)]
pub fn set_pin_high(pin: u8) {
    let p = get_peripherals();

    unsafe{ p.P0.outset.write(|w| w.bits(1 << pin)); }
}

#[allow(dead_code)]
pub fn set_pin_low(pin: u8) {
    let p = get_peripherals();

    unsafe{ p.P0.outclr.write(|w| w.bits(1 << pin)); }
}

#[allow(dead_code)]
pub fn set_pin_state(pin: u8, state: PinState) {
    unsafe{
        let p = get_peripherals();

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
