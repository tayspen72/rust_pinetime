//==============================================================================
// Notes
//==============================================================================
// mcu::spi.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use core::cell::RefCell;
use cortex_m::interrupt::{free, Mutex};
use nrf52832_pac::spi0;
use crate::config;
use crate::mcu::gpio;
use nrf52832_pac::p0::pin_cnf::DIR_A as DIR;
use nrf52832_pac::p0::pin_cnf::PULL_A as PULL;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[allow(dead_code)]
pub struct SpiLine{
	pub sclk_pin: u8,
	pub mosi_pin: u8,
	pub miso_pin: u8,
	pub frequency: spi0::frequency::FREQUENCY_A,
	pub order: spi0::config::ORDER_A,
	pub cpha: spi0::config::CPHA_A,
	pub cpol: spi0::config::CPOL_A
}

//==============================================================================
// Macros
//==============================================================================


//==============================================================================
// Variables
//==============================================================================
static SPI_LINE: SpiLine = SpiLine {
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
static SPIM_HANDLE: Mutex<RefCell<Option<nrf52832_pac::SPIM0>>> = 
	Mutex::new(RefCell::new(None));

//==============================================================================
// Implementations
//==============================================================================
#[allow(dead_code)]
pub fn init(spi0: nrf52832_pac::SPI0) {
	configure(&spi0);

	free(|cs| SPI_HANDLE.borrow(cs).replace(Some(spi0)));
	// free(|cs| SPIM_HANDLE.borrow(cs).replace(Some(p.SPIM0)));
}

fn configure(spi: &nrf52832_pac::SPI0) {
	spi.enable.write(|w| w.enable().disabled());

	// Configure MOSI pin
	gpio::pin_setup(SPI_LINE.mosi_pin, DIR::OUTPUT, gpio::PinState::PinLow, PULL::DISABLED);
	spi.psel.mosi.write(|w| unsafe { w.bits(SPI_LINE.mosi_pin as u32) });

	// Configure MISO pin
	gpio::pin_setup(SPI_LINE.miso_pin, DIR::INPUT, gpio::PinState::PinHigh, PULL::PULLUP);
	spi.psel.miso.write(|w| unsafe { w.bits(SPI_LINE.miso_pin as u32) });

	// Configure SCLK pin
	gpio::pin_setup(SPI_LINE.sclk_pin, DIR::OUTPUT, gpio::PinState::PinLow, PULL::DISABLED);
	spi.psel.sck.write(|w| unsafe { w.bits(SPI_LINE.sclk_pin as u32) });

	spi.frequency.write(|w| w.frequency().variant(SPI_LINE.frequency));
	spi.config.write(|w| w
		.order().variant(SPI_LINE.order)
		.cpha().variant(SPI_LINE.cpha)
		.cpol().variant(SPI_LINE.cpol)
	);

	spi.enable.write(|w| w.enable().enabled());
}

#[allow(dead_code)]
pub fn tx_block(block: &[u8]) {
	//TODO: Unfinished and untested
	free(|cs| {
		let spim = SPIM_HANDLE.borrow(cs).borrow();
		let spim0 = spim.as_ref().unwrap();
		spim0.txd.maxcnt.write(|w| unsafe { w.maxcnt().bits(block.len() as u8) });
		spim0.txd.ptr.write(|w| unsafe { w.ptr().bits( block.as_ptr() as u32) });
	});
}

#[allow(dead_code)]
pub fn tx_byte(byte: u8) {
	free(|cs| {
		let spi = SPI_HANDLE.borrow(cs).borrow();
		let spi0 = spi.as_ref().unwrap();

		spi0.txd.write(|w| unsafe { w.txd().bits(byte) });

		while spi0.events_ready.read().bits() == 0 {};

		spi0.rxd.read().bits();
	});
}

#[allow(dead_code)]
pub fn tx_data(data: &[u8]) {
	free(|cs| {
		let spi = SPI_HANDLE.borrow(cs).borrow();
		let spi0 = spi.as_ref().unwrap();

		for i in 0..data.len() {
			spi0.txd.write(|w| unsafe { w.txd().bits(data[i]) });

			while spi0.events_ready.read().bits() == 0 {};

			spi0.rxd.read().bits();
		}
	});
}

//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
