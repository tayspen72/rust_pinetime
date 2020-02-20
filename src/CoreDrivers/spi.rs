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
use core::f32::MAX;


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
pub const MAX_TRANSFER_SIZE: usize = 0x7F;
static mut RECEIVE_BUFFER: [u8; MAX_TRANSFER_SIZE] = [ 0; MAX_TRANSFER_SIZE];
static mut _SPI_STATE: SpiState = SpiState::Uninitialized;

//=========================================================================
// Implementations
//=========================================================================
pub fn init(p: &nrf52832_pac::Peripherals){
    let spim = &p.SPIM0;

    //define pins used
    unsafe {
        spim.psel.sck.write(|w| w.pin().bits(config::SPI_SCLK));
        spim.psel.mosi.write(|w| w.pin().bits(config::SPI_MOSI));
        spim.psel.miso.write(|w| w.pin().bits(config::SPI_MISO));
    }

    //define peripheral config
    spim.frequency.write(|w| w.frequency().m8());
    spim.config.write(|w| w.cpha().trailing());
    spim.config.write(|w| w.cpol().active_low());
    spim.rxd.list.write(|w| w.list().array_list());
    spim.txd.list.write(|w| w.list().array_list());

    //define max transfer sizes
    unsafe {
        spim.txd.maxcnt.write(|w| w.bits(0x7F as u32));
        spim.rxd.maxcnt.write(|w| w.bits(0x7F as u32));
    }

    //configure event flag for spi tx finish (will sed read ready high)
    // spi.intenset.write(|w| w.ready().set());
    
    //update the state flag
    unsafe {_SPI_STATE = SpiState::Ready; }
}

#[allow(dead_code)]
pub fn write_buffer(p: &nrf52832_pac::Peripherals, src: u32, length: usize){
    unsafe {
        if let SpiState::Uninitialized = _SPI_STATE {
            init(p);
        }
        else if let SpiState::Busy = _SPI_STATE {
            return;
        }
    }
    let spim = &p.SPIM0;

    //disable peripheral for pre transfer config
    spim.enable.write(|w| w.enable().disabled());

    unsafe {
        //load buffer address for DMA
        spim.txd.ptr.write(|w| w.ptr().bits(src));
        spim.rxd.ptr.write(|w| w.ptr().bits(RECEIVE_BUFFER.as_ptr() as usize as u32));
        //update transfer size
        spim.txd.maxcnt.write(|w| w.bits(length as u32));
    }

    //reneable for transfer
    spim.enable.write(|w| w.enable().enabled());

    //begin transfer
    unsafe { spim.tasks_start.write(|w| w.bits(1)); }
}


//=========================================================================
// TaskHandler
//=========================================================================
// pub fn task_handler(_p: &nrf52832_pac::Peripherals){
// }


//=========================================================================
// Interrupt
//=========================================================================

