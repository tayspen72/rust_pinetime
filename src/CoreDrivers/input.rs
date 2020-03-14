/*
 * input.rs
 *
 * Created: 12 Feb 2020
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
use crate::config;
use crate::mcu;
use nrf52832_pac;
#![feature(alloc)]
extern crate alloc;
use alloc::vec::Vec;


//=========================================================================
// Mods
//=========================================================================


//=========================================================================
// Types
//=========================================================================
pub struct InputLine{
    state: State,
    pin: u8,
    options: Options,
    timestamp: u32,
    // nextInputLine: NextInputLine
}

pub struct Options{
    pub EdgeTrigger: EdgeOptions,
    pub Pull: PullOptions,
    pub RealTimeCallback: u8
}

pub enum EdgeOptions{
    RisingEdge,
    FallingEdge,
    BothEdges,
    None
}

pub enum PullOptions{
    Disabled,
    PullDown,
    PullUp
}

pub enum State{
    InputUninitialized,
    PinHigh,
    PinLow,
    InputPaused
}


//=========================================================================
// Variables
//=========================================================================
static mut InputVector: Vec::new();


//=========================================================================
// Implementations
//=========================================================================
pub const fn InputLineCreateNew(state: State, pin: u8, options: Options, timestamp: u32, /*next: NextInputLine*/) -> InputLine {
    InputLine {
        state: state,
        pin: pin,
        options: options,
        timestamp: timestamp,
        // nextInputLine: next
    }
}

pub fn init(input: &InputLine) {
    let p = mcu::get_peripherals();
    let p0 = &p.P0;
    let gpiote = &p.GPIOTE;

    //configure the pin
    p0.pin_cnf[input.pin as usize].write(|w| w.dir().input());
    p0.pin_cnf[input.pin as usize].modify( |_, w| w.input().connect());
    match &input.options.Pull{
        PullOptions::Disabled => p0.pin_cnf[input.pin as usize].modify(|_, w| w.pull().disabled()),
        PullOptions::PullDown => p0.pin_cnf[input.pin as usize].modify(|_, w| w.pull().pulldown()),
        PullOptions::PullUp => p0.pin_cnf[input.pin as usize].modify(|_, w| w.pull().pullup())
    }

    //configure the input event trigger
    let event = get_available_event_number(&p);
    gpiote.config[event].write(|w| w.mode().event());
    unsafe { gpiote.config[event].modify(|_, w| w.psel().bits(input.pin)); }
    match &input.options.EdgeTrigger {
        EdgeOptions::None => gpiote.config[event].modify(|_, w| w.polarity().none()),
        EdgeOptions::RisingEdge => gpiote.config[event].modify(|_, w| w.polarity().lo_to_hi()),
        EdgeOptions::FallingEdge=> gpiote.config[event].modify(|_, w| w.polarity().hi_to_lo()),
        EdgeOptions::BothEdges => gpiote.config[event].modify(|_, w| w.polarity().toggle())
    }
    unsafe {
        gpiote.intenset.modify(|_, w| w.bits(1 << input.pin));
        gpiote.intenclr.modify(|_, w| w.bits(1 << input.pin));
    }
}

fn get_available_event_number(p: &nrf52832_pac::Peripherals) -> usize{
    let gpiote = &p.GPIOTE;

    let mut num: usize = 0;
    for i in 0..8{
        if gpiote.config[i].read().mode().is_disabled() {
            num = i;
            break;
        }
    }

    num
}

//=========================================================================
// TaskHandler
//=========================================================================
pub fn task_handler() {

}


//=========================================================================
// Interrupt
//=========================================================================
fn GPIOTE_IRQHandler(){
    // let mut flags: u32 = 0;
    // for i in 0..32{
        //check NRF GPIOTE->Events IN[i] for anything
        //store in flags
}
