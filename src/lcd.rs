use crate::nrf52_mcu as cpu;

pub const LCD_CS: u8 = 25;

pub fn init_lcd(p: &nrf52832_pac::Peripherals){
    //in master mode, cs is standard io. Init as output with state high
    cpu::init_pin(&p, LCD_CS, cpu::io::PinDirection::PinOutput, cpu::io::PinState::PinHigh);

    cpu::spi::init_spi(&p); 
}