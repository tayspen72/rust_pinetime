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


//=========================================================================
// Crates
//=========================================================================
use crate::config;
//use crate::mcu;
use nrf52832_pac;


//=========================================================================
// Mods
//=========================================================================


//=========================================================================
// Types
//=========================================================================
#[allow(dead_code)]
pub enum SpiState{
    Uninitialized,
    Ready,
    Busy,
//    Fault
}


//=========================================================================
// Variables
//=========================================================================
static mut _SPI_STATE: SpiState = SpiState::Uninitialized;


//=========================================================================
// Implementations
//=========================================================================
pub fn init(){
    let p = nrf52832_pac::Peripherals::take().unwrap();
    let spim = &p.SPIM0;

    //define pins used
    unsafe { spim.psel.sck.write(|w| w.pin().bits(config::SPI_SCLK)); }
    unsafe { spim.psel.mosi.write(|w| w.pin().bits(config::SPI_MOSI)); }
    unsafe { spim.psel.miso.write(|w| w.pin().bits(config::SPI_MISO)); }

    //define peripheral config
    spim.frequency.write(|w| w.frequency().m8());
    spim.config.write(|w| w.cpha().trailing());
    spim.config.write(|w| w.cpol().active_low());

    //configure event flag for spi tx finish (will sed read ready high)
    // spi.intenset.write(|w| w.ready().set());
    
    //enable peripheral when finished
    spim.enable.write(|w| w.enable().enabled());

    //update the state flag
    unsafe {_SPI_STATE = SpiState::Ready; }
}

#[allow(dead_code)]
pub fn write_buffer(_src: *const [u8], _dest: *const [u8], _length: u32){
    let p = nrf52832_pac::Peripherals::take().unwrap();
    let _spim = &p.SPIM0;

    unsafe {
        if let SpiState::Uninitialized = _SPI_STATE {
            init();
        }
        else if let SpiState::Busy = _SPI_STATE {
            return;
        }
    }

    //load buffer address for DMA
    //TODO: Look at how the nrf52_pac associates registers with address values
    //p.SPIM0.txd.ptr.write(|w| w.bits(0x0));

}


//=========================================================================
// TaskHandler
//=========================================================================
// pub fn task_handler(_p: &nrf52832_pac::Peripherals){
// }


//=========================================================================
// Interrupt
//=========================================================================

