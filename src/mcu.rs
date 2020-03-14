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
pub enum PinPullUpDown {
    Disabled,
    PullDown,
    PullUp
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
use cortex_m_rt::exception;
#[allow(unused_imports)]
use cortex_m;
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
static mut PERIPHERALS: Option<nrf52832_pac::Peripherals> = None;
static mut CORE_PERIPHERALS: Option<cortex_m::Peripherals> = None;
static mut SYSTICK: bool = false;

//=========================================================================
// Implementations
//=========================================================================
//TODO: Finish this
pub fn init() {
    unsafe {
        if let None = PERIPHERALS{
            PERIPHERALS = Some(nrf52832_pac::Peripherals::take().unwrap());
        }

        if let None = CORE_PERIPHERALS{
            CORE_PERIPHERALS = Some(cortex_m::Peripherals::take().unwrap());
        }

        init_systick();
    }
}

fn init_systick(){
    let syst = &get_core_peripherals().SYST;

    unsafe {
        syst.csr.modify(|v| v | 4);             // write clock source as core
        syst.rvr.write(10_000_000);                  // set reload value
        syst.cvr.write(0);                        // clear current count
        syst.csr.modify(|v| v | 1);             // enable counter
        syst.csr.modify(|v| v | 2);             // enable interrupt
    }
}

pub fn get_peripherals() -> &'static nrf52832_pac::Peripherals{
    unsafe { &PERIPHERALS.unwrap() }
}

#[allow(dead_code)]
pub fn get_core_peripherals() -> &'static cortex_m::Peripherals{
    unsafe { &CORE_PERIPHERALS.unwrap() }
}

#[allow(dead_code)]
pub fn pin_setup(pin: u8, dir: PinDirection, state: PinState, pull: PinPullUpDown){
    unsafe{
        let p = get_peripherals();

        //set pin direction
        match dir {
            PinDirection::PinInput => {
                p.P0.pin_cnf[pin as usize].modify(|_, w| w.dir().input());
                p.P0.pin_cnf[pin as usize].modify(|_, w| w.input().connect());
                match pull{
                    PinPullUpDown::PullDown => p.P0.pin_cnf[pin as usize].modify(|_, w| w.pull().pulldown()),
                    PinPullUpDown::PullUp => p.P0.pin_cnf[pin as usize].modify(|_, w| w.pull().pullup()),
                    _ => ()
                }

            },

            PinDirection::PinOutput => {
                p.P0.pin_cnf[pin as usize].modify(|_, w| w.dir().output());
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
    unsafe{
        let p = get_peripherals();
        p.P0.outset.write(|w| w.bits(1 << pin));
    }
}

#[allow(dead_code)]
pub fn set_pin_low(pin: u8) {
    unsafe{
        let p = get_peripherals();
        p.P0.outclr.write(|w| w.bits(1 << pin));
    }
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
pub fn task_handler(){
    unsafe{
        while SYSTICK == false {
            ();
        }
    }
}


//=========================================================================
// Interrupt
//=========================================================================
#[exception]
fn SysTick() {
    unsafe { SYSTICK = true; }
}
