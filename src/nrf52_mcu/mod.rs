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

const SYSTICK_FREQUENCY: u32 = 80_000;  //10ms 

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
pub fn spi_get_state() -> PeripheralState{
    spi::get_state()
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
