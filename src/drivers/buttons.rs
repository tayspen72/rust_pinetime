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
use crate::core;


//=========================================================================
// Types
//=========================================================================


//=========================================================================
// Variables
//=========================================================================
static mut BUTTON_0: core::input::InputLine = {
    core::input::InputLineCreateNew(
        core::input::State::InputUninitialized,
        config::BUTTON[0],
        core::input::Options {
            EdgeTrigger: core::input::EdgeOptions::FallingEdge,
            Pull: core::input::PullOptions::PullUp,
            RealTimeCallback: 0
        },
        0)
};
// static mut BUTTON_1: core::input::InputLine = {
//     core::input::InputLineCreateNew(
//         core::input::State::InputUninitialized,
//         config::BUTTON[1],
//         core::input::Options {
//             RisingEdgeOnly: 0,
//             FallingEdgeOnly: 0,
//             RealTimeCallback: 0
//         },
//         0)
// };
// static mut BUTTON_2: core::input::InputLine = {
//     core::input::InputLineCreateNew(
//         core::input::State::InputUninitialized,
//         config::BUTTON[2],
//         core::input::Options {
//             RisingEdgeOnly: 0,
//             FallingEdgeOnly: 0,
//             RealTimeCallback: 0
//         },
//         0)
// };
// static mut BUTTON_3: core::input::InputLine = {
//     core::input::InputLineCreateNew(
//         core::input::State::InputUninitialized,
//         config::BUTTON[3],
//         core::input::Options {
//             RisingEdgeOnly: 0,
//             FallingEdgeOnly: 0,
//             RealTimeCallback: 0
//         },
//         0)
// };


//=========================================================================
// Implementations
//=========================================================================
pub fn init() {
    unsafe { core::input::init(&BUTTON_0); }

    // for i in 0..4 {
    //     //init buttons
    //
    //     mcu::pin_setup(p, config::BUTTON[i], mcu::PinDirection::PinInput, mcu::PinState::NA);
    //     //init corresponding LEDs
    //     mcu::pin_setup(p, config::LED[i], mcu::PinDirection::PinOutput, mcu::PinState::PinHigh);
    // }
}

//=========================================================================
// TaskHandler
//=========================================================================
pub fn task_handler() {
    for i in 0..=3 {
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
