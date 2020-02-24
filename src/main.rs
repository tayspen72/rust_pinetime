/*
 * main.rs
 *
 * Created: 21 Jan 2020
 * Author: T. Spencer
 */

//=========================================================================
// Notes
//=========================================================================


//=========================================================================
// Definitions
//=========================================================================
#![no_main]
#![no_std]


//=========================================================================
// Crates
//=========================================================================
// dev profile: easier to debug panics; can put a breakpoint on `rust_begin_unwind`
#[cfg(debug_assertions)]
extern crate panic_halt;

// release profile: minimize the binary size of the application
#[cfg(not(debug_assertions))]
extern crate panic_abort;

use cortex_m_rt::entry;


//=========================================================================
// Mods
//=========================================================================
mod config;
#[allow(non_snake_case)]
#[allow(unused_imports)]
mod CoreDrivers;
#[allow(unused_imports)]
use CoreDrivers as core;

mod drivers;
mod mcu;


//=========================================================================
// Types
//=========================================================================


//=========================================================================
// Variables
//=========================================================================


//=========================================================================
// Implementations
//=========================================================================
#[entry]
fn main() -> ! {
    let cp = &mcu::get_core_peripherals();
    let p = &mcu::get_peripherals();

    mcu::init(cp, p);

    app_init(cp, p);

    loop {
        app_task_handler(cp, p);
    }
}

fn app_init(_cp: &cortex_m::Peripherals, p: &nrf52832_pac::Peripherals){
    drivers::buttons::init(p);
    drivers::lcd::init(p);
}


//=========================================================================
// TaskHandler
//=========================================================================
fn app_task_handler(_cp: &cortex_m::Peripherals, p: &nrf52832_pac::Peripherals){
    drivers::buttons::task_handler(p);
}


//=========================================================================
// Interrupt
//=========================================================================
