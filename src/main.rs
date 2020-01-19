#![no_main]
#![no_std]

// dev profile: easier to debug panics; can put a breakpoint on `rust_begin_unwind`
#[cfg(debug_assertions)]
extern crate panic_halt;

// release profile: minimize the binary size of the application
#[cfg(not(debug_assertions))]
extern crate panic_abort;

use cortex_m_rt::{entry, exception};

mod nrf52_mcu;
use nrf52_mcu as mcu;
mod lcd;
mod flash;
mod task;

static mut SYSTICK_ELAPSED: bool = false;

#[entry]
fn main() -> ! {
    let p = mcu::take_peripherals();

   mcu::init_system(&p);

    loop {
        unsafe { 
            if SYSTICK_ELAPSED{
                SYSTICK_ELAPSED = false;
                task::task_handler(&p);
            }
        }
    }
}

#[exception]
fn SysTick(){
    unsafe { SYSTICK_ELAPSED = true; }
}