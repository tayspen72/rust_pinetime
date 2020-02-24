/*
 * buttons.rs
 *
 * Created:  12 Feb 2020
 * Author: T. Spencer
 */

//=========================================================================
// Notes
//=========================================================================


//=========================================================================
// Definitions
//=========================================================================


//=========================================================================
// Crates
//=========================================================================


//=========================================================================
// Mods
//=========================================================================
use crate::config;
use crate::mcu;

//=========================================================================
// Types
//=========================================================================


//=========================================================================
// Variables
//=========================================================================


//=========================================================================
// Implementations
//=========================================================================
pub fn init() {
    for i in 0..4 {
        //init buttons
        mcu::pin_setup(config::BUTTON[i], mcu::PinDirection::PinInput, mcu::PinState::NA);
        //init corresponding LEDs
        mcu::pin_setup(config::LED[i], mcu::PinDirection::PinOutput, mcu::PinState::PinHigh);
    }
}

//=========================================================================
// TaskHandler
//=========================================================================
pub fn task_handler() {
    for i in 0..4 {
        match mcu::get_pin_state(config::BUTTON[i]) {
            //if low (button pressed) - set led pin low (on)
            mcu::PinState::PinLow => mcu::set_pin_low(config::LED[i]),
            //if high (button not pressed) - set led pin high (off)
            mcu::PinState::PinHigh => mcu::set_pin_high(config::LED[i]),
            //else, no response
            _ => (),
        };
    }
}

//=========================================================================
// Interrupt
//=========================================================================

