//==============================================================================
// Notes
//==============================================================================
// mcu::spi.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use core::cell::{Cell, RefCell};
use core::ops::DerefMut;
use cortex_m::interrupt::{free, Mutex};
use nrf52832_pac::spi0;
use crate::config;
use crate::mcu::gpio;
use nrf52832_pac::p0::pin_cnf::DIR_A as DIR;
use nrf52832_pac::p0::pin_cnf::PULL_A as PULL;
use core::ptr;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[derive(Clone, Copy)]
enum ActiveBank {
	BankA,
	BankB
}

pub struct SpiLine{
	pub sclk_pin: u8,
	pub mosi_pin: u8,
	pub miso_pin: u8,
	pub frequency: spi0::frequency::FREQUENCY_A,
	pub order: spi0::config::ORDER_A,
	pub cpha: spi0::config::CPHA_A,
	pub cpol: spi0::config::CPOL_A
}

type ArrayList = [u8];

pub enum SpiError{
	DMA,
	HANDLER,
	RECEIVE, 
	TRANSMIT,
}

//==============================================================================
// Variables
//==============================================================================
const SPIM_ACTIVE_BANK: Mutex<Cell<ActiveBank>> = Mutex::new(Cell::new(ActiveBank::BankB));

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
static SPIM_HANDLE: Mutex<RefCell<Option<nrf52832_pac::SPIM0>>> = 
	Mutex::new(RefCell::new(None));

//==============================================================================
// Public Functions
//==============================================================================
pub fn init(spi0: nrf52832_pac::SPI0, spim0: nrf52832_pac::SPIM0) {
	configure(&spi0, &spim0);

	free(|cs| SPI_HANDLE.borrow(cs).replace(Some(spi0)));
	free(|cs| SPIM_HANDLE.borrow(cs).replace(Some(spim0)));
}

pub fn write_data(data: &ArrayList) {
	let mut num_bytes = data.len();
	let mut index = 0;

	while num_bytes > 0 {
		let transfer_size = if num_bytes > 256 { 256 } else { num_bytes };
		num_bytes = num_bytes - transfer_size;
	
		tx_data(&data[index..index+transfer_size]);

		index = index + transfer_size;
	}
}

pub fn write_data_solid(color: u16, len: u32) {
	// build a single block and setup the DMA once
	// let color = color.to_le_bytes();
	// let mut block: [u8; 256] = [0; 256];
	// for word in block.chunks_exact_mut(2) {
	// 	word[0] = color[1];
	// 	word[1] = color[0];
	// }

	// build a single block and setup the DMA once
	// let mut block: [u16; 128] = [color; 128];
	// let block = unsafe {
	// 	core::slice::from_raw_parts_mut(block.as_mut_ptr() as *mut u8, block.len() * 2)
	// };

	// Build a single block and setup the DMA once
	let block: [u16; 128] = [color; 128];
	let block = unsafe {
		core::mem::transmute::<[u16; 128], [u8; 256]>(block)
	};

	// Need to be sending 2B per pixel
	let mut num_bytes = len * 2;

	while num_bytes > 0 {
		let transfer_size = if num_bytes > 256 { 256 } else { num_bytes };
		num_bytes = num_bytes - transfer_size;

		write_data(&block[0..(transfer_size) as usize], false);
	}
}

//==============================================================================
// Private Functions
//==============================================================================
fn configure(spi: &nrf52832_pac::SPI0, spim: &nrf52832_pac::SPIM0) {
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

	// Ensure we are using the ArrayList structure
	spim.rxd.list.write(|w| w.list().variant(nrf52832_pac::spim0::rxd::list::LIST_A::ARRAYLIST));
	spim.txd.list.write(|w| w.list().variant(nrf52832_pac::spim0::txd::list::LIST_A::ARRAYLIST));

	spi.enable.write(|w| w.enable().enabled());
}

fn tx_byte(spi: &nrf52832_pac::SPI0, byte: u8) -> Result<(), SpiError> {

	Ok(())
}

//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
