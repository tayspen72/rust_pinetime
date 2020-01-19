use crate::nrf52_mcu as cpu;

pub const FLASH_CS: u8 = 5;

pub fn init_flash(p: &nrf52832_pac::Peripherals){
    //in master mode, cs is standard io. Init as output with state high
    cpu::init_pin(&p, FLASH_CS, cpu::io::PinDirection::PinOutput, cpu::io::PinState::PinHigh);

    // if let cpu::spi_get_state() == cpu::PeripheralState::Ready{
        cpu::spi::init_spi(&p);
    // }
}