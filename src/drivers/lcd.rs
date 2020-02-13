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
use crate::config;
//use crate::CoreDrivers as core;
use crate::mcu;



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


//=========================================================================
// Implementations
//=========================================================================
pub fn init(){
    //in master mode, cs is standard io. Init as output with state high
    mcu::pin_setup(config::LCD_CS, mcu::PinDirection::PinOutput, mcu::PinState::PinHigh);
    //reset pin must be held high for operation
    mcu::pin_setup(config::LCD_RESET, mcu::PinDirection::PinOutput, mcu::PinState::PinHigh);
    //init lcd backlight pins
    mcu::pin_setup(config::LCD_BACKLIGHT[0], mcu::PinDirection::PinOutput, mcu::PinState::PinLow);
    mcu::pin_setup(config::LCD_BACKLIGHT[1], mcu::PinDirection::PinOutput, mcu::PinState::PinLow);
    mcu::pin_setup(config::LCD_BACKLIGHT[2], mcu::PinDirection::PinOutput, mcu::PinState::PinHigh);

    set_backlight(BacklightBrightness::Brightness3);

    //init spi peripheral
//    mcu::spi::init(&p);
}

pub fn set_backlight(val: BacklightBrightness){
    let val = val as u8;

    //set Backlight pin 3
    if val & 0x4 > 0 {
        mcu::set_pin_high(config::LCD_BACKLIGHT[2]);
    }
    else{
        mcu::set_pin_low(config::LCD_BACKLIGHT[2]);
    }

    //set Backlight pin 2
    if val & 0x2 > 0 {
        mcu::set_pin_high(config::LCD_BACKLIGHT[1]);
    }
    else{
        mcu::set_pin_low(config::LCD_BACKLIGHT[1]);
    }


    //set Backlight pin 1
    if val & 0x1 > 0 {
        mcu::set_pin_high(config::LCD_BACKLIGHT[0]);
    }
    else{
        mcu::set_pin_low(config::LCD_BACKLIGHT[0]);
    }
}

//pub fn write_display_buffer()
//{
//    core::spi::write_buffer(1, 2, 3);
//}

//=========================================================================
// TaskHandler
//=========================================================================
//pub fn task_handler(_p: &nrf52832_pac::Peripherals){
//
//}


//=========================================================================
// Interrupt
//=========================================================================

