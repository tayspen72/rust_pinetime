/*
 * lcd.rs
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


//=========================================================================
// Crates
//=========================================================================


//=========================================================================
// Mods
//=========================================================================
use crate::nrf52_mcu as mcu;


//=========================================================================
// Types
//=========================================================================
#[allow(dead_code)]
pub enum BacklightBrightness{
    Brightness0   = 0x0,
    Brightness1   = 0x1,
    Brightness2   = 0x2,
    Brightness3   = 0x3,
    Brightness4   = 0x4,
    Brightness5   = 0x5,
    Brightness6   = 0x6,
    Brightness7   = 0x7
}


//=========================================================================
// Variables
//=========================================================================
static mut _LCD_STATE: mcu::PeripheralState = mcu::PeripheralState::Uninitialized;


//=========================================================================
// Implementations
//=========================================================================
pub fn init(p: &nrf52832_pac::Peripherals){
    //in master mode, cs is standard io. Init as output with state high
    mcu::io::pin_init(&p, LCD_CS, mcu::io::PinDirection::PinOutput, mcu::io::PinState::PinHigh);
    //reset pin must be held high for operation
    mcu::io::pin_init(&p, LCD_RESET, mcu::io::PinDirection::PinOutput, mcu::io::PinState::PinHigh);
    //init lcd backlight pins
    mcu::io::pin_init(&p, LCD_BACKLIGHT1, mcu::io::PinDirection::PinOutput, mcu::io::PinState::PinLow);
    mcu::io::pin_init(&p, LCD_BACKLIGHT2, mcu::io::PinDirection::PinOutput, mcu::io::PinState::PinLow);
    mcu::io::pin_init(&p, LCD_BACKLIGHT3, mcu::io::PinDirection::PinOutput, mcu::io::PinState::PinHigh);

    //init spi peripheral
    mcu::spi::init(&p);
}

pub fn set_backlight(p: &nrf52832_pac::Peripherals, val: BacklightBrightness){
    unsafe {
        if let mcu::PeripheralState::Uninitialized = _LCD_STATE{
            init(&p);
        }
    }
    
    let val = val as u8;

    //set Backlight pin 3
    if val & 0x4 > 0 {
        mcu::io::pin_set(&p, LCD_BACKLIGHT3, mcu::io::PinState::PinHigh);
    }
    else{
        mcu::io::pin_set(&p, LCD_BACKLIGHT3, mcu::io::PinState::PinLow);
    }

    //set Backlight pin 2
    if val & 0x2 > 0 {
        mcu::io::pin_set(&p, LCD_BACKLIGHT2, mcu::io::PinState::PinHigh);
    }
    else{
        mcu::io::pin_set(&p, LCD_BACKLIGHT2, mcu::io::PinState::PinLow);
    }

    //set Backlight pin 1
    if val & 0x1 > 0 {
        mcu::io::pin_set(&p, LCD_BACKLIGHT1, mcu::io::PinState::PinHigh);
    }
    else{
        mcu::io::pin_set(&p, LCD_BACKLIGHT1, mcu::io::PinState::PinLow);
    }
}

pub fn write_byte(p: &nrf52832_pac::Peripherals, val: u8)
{
    mcu::spi::write_byte(&p, LCD_CS, val);
}

//=========================================================================
// TaskHandler
//=========================================================================
pub fn task_handler(_p: &nrf52832_pac::Peripherals){
    
}


//=========================================================================
// Interrupt
//=========================================================================

