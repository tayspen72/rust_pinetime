//==============================================================================
// Notes
//==============================================================================
// mcu::spi.rs

//==============================================================================
// Crates and Mods
//==============================================================================
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

static mut _INITIALIZED: bool = false;

//==============================================================================
// Implementations
//==============================================================================
#[allow(dead_code)]
pub fn init(p: &nrf52832_pac::Peripherals) {
	let spi = &p.SPI0;

	spi.enable.write(|w| w.enable().disabled());

	// Configure MOSI pin
	gpio::pin_setup(p, SPI_LINE.mosi_pin, DIR::OUTPUT, gpio::PinState::PinLow, PULL::DISABLED);
	spi.psel.mosi.write(|w| unsafe { w.bits(SPI_LINE.mosi_pin as u32) });

	// Configure MISO pin
	gpio::pin_setup(p, SPI_LINE.miso_pin, DIR::INPUT, gpio::PinState::PinHigh, PULL::PULLUP);
	spi.psel.miso.write(|w| unsafe { w.bits(SPI_LINE.miso_pin as u32) });

	// Configure SCLK pin
	gpio::pin_setup(p, SPI_LINE.sclk_pin, DIR::OUTPUT, gpio::PinState::PinLow, PULL::DISABLED);
	spi.psel.sck.write(|w| unsafe { w.bits(SPI_LINE.sclk_pin as u32) });

	spi.frequency.write(|w| w.frequency().variant(SPI_LINE.frequency));
	spi.config.write(|w| w
		.order().variant(SPI_LINE.order)
		.cpha().variant(SPI_LINE.cpha)
		.cpol().variant(SPI_LINE.cpol)
	);

	spi.enable.write(|w| w.enable().enabled());

	unsafe { _INITIALIZED = true };
}

#[allow(dead_code)]
pub fn tx_block(p: &nrf52832_pac::Peripherals, block: &[u8]) {
	if !unsafe { _INITIALIZED }{
		init(p);
	}

	p.SPIM0.txd.maxcnt.write(|w| unsafe { w.maxcnt().bits(block.len() as u8) });
	p.SPIM0.txd.ptr.write(|w| unsafe { w.ptr().bits( block.as_ptr() as u32) });
}

#[allow(dead_code)]
pub fn tx_byte(p: &nrf52832_pac::Peripherals, byte: u8) {
	if !unsafe { _INITIALIZED }{
		init(p);
	}

	p.SPI0.txd.write(|w| unsafe { w.txd().bits(byte) });

	while p.SPI0.events_ready.read().bits() == 0 {};

	p.SPI0.rxd.read().bits();
}

#[allow(dead_code)]
pub fn tx_data(p: &nrf52832_pac::Peripherals, data: &[u8]) {
	if !unsafe { _INITIALIZED }{
		init(p);
	}

	for i in 0..data.len() {
		p.SPI0.txd.write(|w| unsafe { w.txd().bits(data[i]) });

		while p.SPI0.events_ready.read().bits() == 0 {};

		p.SPI0.rxd.read().bits();
	}
}

//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
