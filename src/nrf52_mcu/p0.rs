/*
 * p0.rs
 *
 * Created: 21 Jan 2020
 * Author: T. Spencer
 */

//=========================================================================
// Notes
//=========================================================================
//P0 and IO are used interchangable within the project

//=========================================================================
// Definitions
//=========================================================================


//=========================================================================
// Crates
//=========================================================================


//=========================================================================
// Mods
//=========================================================================


//=========================================================================
// Types
//=========================================================================
#[allow(dead_code)]
pub enum PinDirection {
    PinInput,
    PinOutput
}

#[allow(dead_code)]
pub enum PinState {
    PinLow,
    PinHigh
}


//=========================================================================
// Variables
//=========================================================================


//=========================================================================
// Implementations
//=========================================================================

#[allow(dead_code)]
pub fn pin_init(p: &nrf52832_pac::Peripherals, pin: u8, dir: PinDirection, state: PinState){
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
            PinState::PinHigh => p.P0.outset.write(|w| w.bits(1 << pin))
        };
    };
}


//=========================================================================
// TaskHandler
//=========================================================================
// pub fn task_handler(_p: &nrf52832_pac::Peripherals){
// }


//=========================================================================
// Interrupt
//=========================================================================

