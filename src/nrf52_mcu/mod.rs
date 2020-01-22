/*
 * mod.rs -> nrf52_mcu 
 *
 * Created: 21 Jan 2020
 * Author: T. Spencer
 */

//=========================================================================
// Notes
//=========================================================================
/*
 * This file is meant to provide access to the underlying framework for 
 * accessing the needed peripheral set for the nrf52832 MCU. All sub-modules
 * will be defined and instantiated here as needed.
 * 
 */

//=========================================================================
// Definitions
//=========================================================================


//=========================================================================
// Crates
//=========================================================================


//=========================================================================
// Mods
//=========================================================================
use nrf52832_pac;
pub mod p0;
pub use p0 as io;
pub mod spi;
pub mod uart;

use crate::flash as flash;
use crate::lcd as lcd;


//=========================================================================
// Types
//=========================================================================
#[allow(dead_code)]
pub enum ChipSelect{
    Lcd,
    Flash
}

#[allow(dead_code)]
pub enum PeripheralState{
    Fault,
    Ready,
    Uninitialized
}


//=========================================================================
// Variables
//=========================================================================
const SYSTICK_FREQUENCY: u32 = 80_000;  //10ms 


//=========================================================================
// Implementations
//=========================================================================
pub fn init_system(p: &nrf52832_pac::Peripherals){    
    //init core peripherals
    flash::init(&p);
    lcd::init(&p);
    init_systick();

    //init lcd state
    lcd::set_backlight(&p, lcd::BacklightBrightness::Brightness1);
}

fn init_systick(){
        // config the SysTick timer to fire exception every N cycles
        let p = cortex_m::Peripherals::take().unwrap();
        let mut systick = p.SYST;
        systick.set_clock_source(cortex_m::peripheral::syst::SystClkSource::Core);
        systick.set_reload(SYSTICK_FREQUENCY); // period = 1s
        systick.enable_counter();
        systick.enable_interrupt();
}

#[allow(dead_code)]
pub fn spi_read_byte(p: &nrf52832_pac::Peripherals) -> u8{
    spi::read_byte(&p)
}

#[allow(dead_code)]
pub fn spi_write(p: &nrf52832_pac::Peripherals, cs: ChipSelect, val: u8){
    //get the cs pin
    let cs_pin = match cs {
        ChipSelect::Lcd => lcd::LCD_CS,
        ChipSelect::Flash => flash::FLASH_CS
    };

    //write the byte
    spi::write_byte(&p, cs_pin, val);
}

pub fn take_peripherals() -> nrf52832_pac::Peripherals {
    nrf52832_pac::Peripherals::take().unwrap()
}


//=========================================================================
// TaskHandler
//=========================================================================
// pub fn task_handler(_p: &nrf52832_pac::Peripherals){    
// }


//=========================================================================
// Interrupt
//=========================================================================

