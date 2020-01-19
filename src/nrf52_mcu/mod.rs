use nrf52832_pac;
pub mod p0;
pub mod spi;
pub mod uart;

use crate::flash as flash;
use crate::lcd as lcd;
pub use p0 as io;

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

#[allow(dead_code)]
pub fn init_pin(p: &nrf52832_pac::Peripherals, pin: u8, dir: io::PinDirection, state: io::PinState){
    io::init_pin(p, pin, dir, state);
}

pub fn init_system(p: &nrf52832_pac::Peripherals){    
    flash::init_flash(&p);
    lcd::init_lcd(&p);
}

pub fn take_peripherals() -> nrf52832_pac::Peripherals {
    nrf52832_pac::Peripherals::take().unwrap()
}

#[allow(dead_code)]
pub fn spi_get_state() -> PeripheralState{
    spi::get_state()
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

#[allow(dead_code)]
pub fn spi_read_byte(p: &nrf52832_pac::Peripherals) -> u8{
    spi::read_byte(&p)
}
