use crate::nrf52_mcu as cpu;

pub const LCD_CS: u8 = 25;
pub const LCD_BACKLIGHT1: u8 = 25;
pub const LCD_BACKLIGHT2: u8 = 25;
pub const LCD_BACKLIGHT3: u8 = 25;

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

pub fn init(p: &nrf52832_pac::Peripherals){
    //in master mode, cs is standard io. Init as output with state high
    cpu::io::pin_init(&p, LCD_CS, cpu::io::PinDirection::PinOutput, cpu::io::PinState::PinHigh);
    //init lcd backlight pins
    cpu::io::pin_init(&p, LCD_BACKLIGHT1, cpu::io::PinDirection::PinOutput, cpu::io::PinState::PinLow);
    cpu::io::pin_init(&p, LCD_BACKLIGHT2, cpu::io::PinDirection::PinOutput, cpu::io::PinState::PinLow);
    cpu::io::pin_init(&p, LCD_BACKLIGHT3, cpu::io::PinDirection::PinOutput, cpu::io::PinState::PinLow);

    //init spi peripheral
    //TODO: Look into if let type confitions for enum matching
    // if let cpu::spi::get_state() = cpu::PeripheralState::Uninitialized{
    cpu::spi::init(&p); 
    // }
}

pub fn set_backlight(p: &nrf52832_pac::Peripherals, val: BacklightBrightness){
    let val = val as u8;

    //set Backlight pin 3
    if val & 0x4 > 0 {
        cpu::io::pin_set(&p, LCD_BACKLIGHT3, cpu::io::PinState::PinHigh);
    }
    else{
        cpu::io::pin_set(&p, LCD_BACKLIGHT3, cpu::io::PinState::PinLow);
    }

    //set Backlight pin 2
    if val & 0x2 > 0 {
        cpu::io::pin_set(&p, LCD_BACKLIGHT2, cpu::io::PinState::PinHigh);
    }
    else{
        cpu::io::pin_set(&p, LCD_BACKLIGHT2, cpu::io::PinState::PinLow);
    }

    //set Backlight pin 1
    if val & 0x1 > 0 {
        cpu::io::pin_set(&p, LCD_BACKLIGHT1, cpu::io::PinState::PinHigh);
    }
    else{
        cpu::io::pin_set(&p, LCD_BACKLIGHT1, cpu::io::PinState::PinLow);
    }
}