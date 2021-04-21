//==============================================================================
// Notes
//==============================================================================
// mcu::spi.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use core::cell::RefCell;
use core::ops::DerefMut;
use cortex_m::interrupt::{free, Mutex};
use nrf52832_pac::spi0;
use crate::config;
use crate::mcu::gpio;
use nrf52832_pac::p0::pin_cnf::DIR_A as DIR;
use nrf52832_pac::p0::pin_cnf::PULL_A as PULL;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
pub struct SpiLine{
	pub sclk_pin: u8,
	pub mosi_pin: u8,
	pub miso_pin: u8,
	pub frequency: spi0::frequency::FREQUENCY_A,
	pub order: spi0::config::ORDER_A,
	pub cpha: spi0::config::CPHA_A,
	pub cpol: spi0::config::CPOL_A
}

#[allow(dead_code)]
pub enum SpiError{
	HANDLER,
	RECEIVE, 
	TRANSMIT,
}

//==============================================================================
// Variables
//==============================================================================
const SPI_LINE: SpiLine = SpiLine {
	sclk_pin: config::SPI_SCLK_PIN,
	mosi_pin: config::SPI_MOSI_PIN,
	miso_pin: config::SPI_MISO_PIN,
	frequency: config::SPI_FREQUENCY,
	order: config::SPI_ORDER,
	cpha: config::SPI_CPHA,
	cpol: config::SPI_CPOL,
};

static SPI_HANDLE: Mutex<RefCell<Option<nrf52832_pac::SPI0>>> = 
	Mutex::new(RefCell::new(None));

//==============================================================================
// Public Functions
//==============================================================================
pub fn init(spi0: nrf52832_pac::SPI0) {
	configure(&spi0);

	free(|cs| SPI_HANDLE.borrow(cs).replace(Some(spi0)));
}

pub fn write(buf: &[u8]) -> Result<(), SpiError> {
	free(|cs| {
		if let Some(ref mut spi) = SPI_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			spi.enable.write(|w| w.enable().enabled());

			if let Some((last, data)) = buf.split_last() {
				// If there is a block to be written
				if !data.is_empty() {
					let mut is_first: bool = true;
					for byte in &mut data.into_iter() {
						tx_byte(spi, *byte)?;

						// Skip waiting for receive on the first byte to take use double buffer
						if is_first {
							is_first = false;
						}
						else {
							rx_byte(spi)?;
						}
					}
					// Receive the last byte to clear out the double buffer
					rx_byte(spi)?;
				}

				// Send the last byte
				tx_byte(spi, *last)?;
				rx_byte(spi)?;
			}

			spi.enable.write(|w| w.enable().disabled());
			gpio::set_pin_state(config::SPI_SCLK_PIN, gpio::PinState::PinHigh);

			Ok(())
		}
		else {
			Err(SpiError::HANDLER)
		}
	})
}

#[allow(dead_code)]
pub fn write_u16(data: u16, len: u32) -> Result<(), SpiError> {
	// Build a single block and setup the DMA once
	let block: [u16; 128] = [data; 128];
	let block = unsafe {
		core::mem::transmute::<[u16; 128], [u8; 256]>(block)
	};

	// Need to be sending 2B per pixel
	let mut num_bytes = len * 2;

	while num_bytes > 0 {
		let transfer_size = if num_bytes > 256 { 256 } else { num_bytes };
		num_bytes = num_bytes - transfer_size;

		write(&block[0..(transfer_size) as usize])?;
	}

	Ok(())
}

//==============================================================================
// Private Functions
//==============================================================================
fn configure(spi: &nrf52832_pac::SPI0) {
	spi.enable.write(|w| w.enable().disabled());

	// Configure SCLK pin
	gpio::pin_setup(SPI_LINE.sclk_pin, DIR::OUTPUT, gpio::PinState::PinLow, PULL::DISABLED);
	spi.psel.sck.write(|w| unsafe { w.bits(SPI_LINE.sclk_pin as u32) });

	// Configure MISO pin
	gpio::pin_setup(SPI_LINE.miso_pin, DIR::INPUT, gpio::PinState::PinHigh, PULL::PULLUP);
	spi.psel.miso.write(|w| unsafe { w.bits(SPI_LINE.miso_pin as u32) });

	// Configure MOSI pin
	gpio::pin_setup(SPI_LINE.mosi_pin, DIR::OUTPUT, gpio::PinState::PinLow, PULL::DISABLED);
	spi.psel.mosi.write(|w| unsafe { w.bits(SPI_LINE.mosi_pin as u32) });

	spi.frequency.write(|w| w.frequency().variant(SPI_LINE.frequency));
	spi.config.write(|w| w
		.order().variant(SPI_LINE.order)
		.cpha().variant(SPI_LINE.cpha)
		.cpol().variant(SPI_LINE.cpol)
	);

	gpio::set_pin_state(config::SPI_SCLK_PIN, gpio::PinState::PinHigh);
}

fn rx_byte(spi: &nrf52832_pac::SPI0) -> Result<u8, SpiError> {
	// Wait for byte to be received
	while spi.events_ready.read().bits() == 0 {}

	Ok(spi.rxd.read().bits() as u8)
}

fn tx_byte(spi: &nrf52832_pac::SPI0, byte: u8) -> Result<(), SpiError> {
	spi.txd.write(|w| unsafe { w.bits(byte.into()) });
	Ok(())
}

//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
