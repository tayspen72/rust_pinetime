/*
 * spi.rs
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
const SPI_SCLK: u8 = 2;
const SPI_MOSI: u8 = 3;
const SPI_MISO: u8 = 4;


//=========================================================================
// Crates
//=========================================================================
use super::super::nrf52_mcu as mcu;
use super::p0 as io;


//=========================================================================
// Mods
//=========================================================================


//=========================================================================
// Types
//=========================================================================


//=========================================================================
// Variables
//=========================================================================
static mut _SPI_STATE: mcu::PeripheralState = mcu::PeripheralState::Uninitialized;


//=========================================================================
// Implementations
//=========================================================================
pub fn init(p: &nrf52832_pac::Peripherals){
    let spim = &p.SPIM0;

    //define pins used
    unsafe { spim.psel.sck.write(|w| w.pin().bits(SPI_SCLK)); }
    unsafe { spim.psel.mosi.write(|w| w.pin().bits(SPI_MOSI)); }
    unsafe { spim.psel.miso.write(|w| w.pin().bits(SPI_MISO)); }

    //define peripheral config
    spim.frequency.write(|w| w.frequency().m8());
    spim.config.write(|w| w.cpha().trailing());
    spim.config.write(|w| w.cpol().active_low());

    //configure event flag for spi tx finish (will sed read ready high)
    // spi.intenset.write(|w| w.ready().set());
    
    //enable peripheral when finished
    spim.enable.write(|w| w.enable().enabled());

    //update the state flag
    unsafe {_SPI_STATE = mcu::PeripheralState::Ready; }
}

#[allow(dead_code)]
pub fn write_byte(p: &nrf52832_pac::Peripherals, cs: u8, val: u8){
    unsafe { 
        if let mcu::PeripheralState::Uninitialized = _SPI_STATE {
            init(&p);
        }
    }

    //set the chip select low for spi writing
    io::pin_set(&p, cs, io::PinState::PinLow);

    //write the byte to the peripheral
    unsafe { p.SPI0.txd.write(|w| w.bits(val as u32)); }

//    unsafe { p.SPIM0.tasks_start.write(|w| w.bits(1)); }
    unsafe { p.SPIM0.tasks_start.write(|w| w.bits(0x1)); }

    //read buffer to prevent overflow
    read_byte(&p);

    //set the chip select high when finished
    io::pin_set(&p, cs, io::PinState::PinHigh);
}

#[allow(dead_code)]
pub fn write_byte_dma(p: nrf52832_pac::Peripherals, buf: *const [u8], buf_length: u32, cs: u8, val: u8){
    unsafe {
        if let mcu::PeripheralState::Uninitialized = _SPI_STATE {
            init(&p);
        }
    }

//    let src = &buf as *const i32;

//    unsafe { p.SPIM0.txd.ptr.write(|w| w.bits(buf.into())); }

}

#[allow(dead_code)]
pub fn read_byte(p: &nrf52832_pac::Peripherals) -> u8 {
    unsafe { 
        if let mcu::PeripheralState::Uninitialized = _SPI_STATE {
            init(&p);
        }
    }
    
    p.SPI0.rxd.read().rxd().bits()
}


//=========================================================================
// TaskHandler
//=========================================================================
// pub fn task_handler(_p: &nrf52832_pac::Peripherals){
// }


//=========================================================================
// Interrupt
//=========================================================================

