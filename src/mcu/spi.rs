//==============================================================================
// Notes
//==============================================================================
// mcu::spi.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use nrf52832_pac::spi0;
use crate::mcu::gpio;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[allow(dead_code)]
pub struct SpiLine{
	pub sclk_pin: u8,
	pub sel_pin: u8,
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


//==============================================================================
// Implementations
//==============================================================================
#[allow(dead_code)]
pub fn init(p: &nrf52832_pac::Peripherals, spiline: &SpiLine) {
	let spi = &p.SPI0;

	spi.enable.write(|w| w.enable().disabled());

	// Configure MOSI pin
	gpio::pin_setup(p, spiline.mosi_pin, nrf52832_pac::p0::pin_cnf::DIR_A::OUTPUT, nrf52832_pac::p0::pin_cnf::PULL_A::DISABLED, gpio::PinState::PinLow);
	spi.psel.mosi.write(|w| unsafe { w.bits(spiline.mosi_pin as u32) });

	// Configure MISO pin
	gpio::pin_setup(p, spiline.miso_pin, nrf52832_pac::p0::pin_cnf::DIR_A::OUTPUT, nrf52832_pac::p0::pin_cnf::PULL_A::DISABLED, gpio::PinState::PinLow);
	spi.psel.miso.write(|w| unsafe { w.bits(spiline.miso_pin as u32) });

	// Configure SCLK pin
	spi.psel.sck.write(|w| unsafe { w.bits(spiline.sclk_pin as u32) });

	// Configure SEL pin
	gpio::pin_setup(p, spiline.sel_pin, nrf52832_pac::p0::pin_cnf::DIR_A::OUTPUT, nrf52832_pac::p0::pin_cnf::PULL_A::DISABLED, gpio::PinState::PinHigh);

	spi.frequency.write(|w| w.frequency().variant(spiline.frequency));
	spi.config.write(|w| w
		.order().variant(spiline.order)
		.cpha().variant(spiline.cpha)
		.cpol().variant(spiline.cpol)
	);
}

#[allow(dead_code)]
pub fn tx_byte(p: &nrf52832_pac::Peripherals, spiline: &SpiLine, byte: u8) {
	set_sel(p, spiline, true);

	p.SPI0.txd.write(|w| unsafe { w.txd().bits(byte) });

	while p.SPI0.events_ready.read().bits() != 0 {};

	set_sel(p, spiline, false);

	p.SPI0.rxd.read().bits();
}

#[allow(dead_code)]
pub fn tx_data(p: &nrf52832_pac::Peripherals, spiline: &SpiLine, data: &[u8]) {
	set_sel(p, spiline, true);

	for i in 0..(data.len()-1) {
		p.SPI0.txd.write(|w| unsafe { w.txd().bits(data[i]) });

		while p.SPI0.events_ready.read().bits() != 0 {};

		set_sel(p, spiline, false);

		p.SPI0.rxd.read().bits();
	}
}

#[allow(dead_code)]
fn set_sel(p: &nrf52832_pac::Peripherals, spiline: &SpiLine, is_transferring: bool) {
	if is_transferring {
		gpio::set_pin_state(p, spiline.sel_pin, gpio::PinState::PinLow);
	}
	else {
		gpio::set_pin_state(p, spiline.sel_pin, gpio::PinState::PinHigh);
	}
}

//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
