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
    mcu::init();

    app_init();

    loop {
        app_task_handler();
    }
}

fn app_init(){
    drivers::buttons::init();
}


//=========================================================================
// TaskHandler
//=========================================================================
fn app_task_handler(){
    mcu::task_handler();
    drivers::buttons::task_handler();
}


//=========================================================================
// Interrupt
//=========================================================================
