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
    PinLow = 0,
    PinHigh = 1,
    NA = 2,
}


//=========================================================================
// Crates
//=========================================================================
use nrf52832_pac;


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

}

#[allow(dead_code)]
pub fn pin_init(pin: u8, dir: PinDirection, state: PinState){
    let p =  nrf52832_pac::Peripherals::take().unwrap();

    //set pin direction
    match dir{
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
                    _ => (),
                };
            }
        }
    };
}

#[allow(dead_code)]
pub fn pin_set(p: &nrf52832_pac::Peripherals, pin: u8, state: PinState){
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

