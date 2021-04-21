//==============================================================================
// Notes
//==============================================================================
// mcu::spi.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use core::cell::RefCell;
use core::ops::DerefMut;
use core::ptr;
use cortex_m::interrupt::{free, Mutex};
use nrf52832_pac::spim0;
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
	pub frequency: spim0::frequency::FREQUENCY_A,
	pub order: spim0::config::ORDER_A,
	pub cpha: spim0::config::CPHA_A,
	pub cpol: spim0::config::CPOL_A
}

pub enum SpimError{
	Handler,
	Receive, 
	Transmit,
}

//==============================================================================
// Variables
//==============================================================================
const SPIM_LINE: SpiLine = SpiLine {
	sclk_pin: config::SPI_SCLK_PIN,
	mosi_pin: config::SPI_MOSI_PIN,
	miso_pin: config::SPI_MISO_PIN,
	frequency: config::SPIM_FREQUENCY,
	order: config::SPIM_ORDER,
	cpha: config::SPIM_CPHA,
	cpol: config::SPIM_CPOL,
};

static SPIM_HANDLE: Mutex<RefCell<Option<nrf52832_pac::SPIM0>>> = 
	Mutex::new(RefCell::new(None));

//==============================================================================
// Public Functions
//==============================================================================
pub fn init(spim0: nrf52832_pac::SPIM0) {
	configure(&spim0);

	free(|cs| SPIM_HANDLE.borrow(cs).replace(Some(spim0)));
}

#[allow(dead_code)]
pub fn write(tx_block: &[u8]) -> Result<(), SpimError> {
	let rx_ptr = config::SPIM_DMA_RX_PTR;
	let (tx_ptr, len) = make_ram(tx_block);

	free(|cs| {
		if let Some(ref mut spim) = SPIM_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			spim.enable.write(|w| w.enable().enabled());

			// Setup the DMA transfer
			spim.txd.ptr.write(|w| unsafe { w.ptr().bits(tx_ptr) });
			spim.txd.maxcnt.write(|w| 
				unsafe { w.maxcnt().bits(len as u8) });
			spim.rxd.ptr.write(|w| unsafe { w.ptr().bits(rx_ptr) });
			spim.rxd.maxcnt.write(|w|
				unsafe { w.maxcnt().bits(len as u8) });
	
			// Clear the end flag in case it wasn't properly cleared last time
			spim.events_end.write(|w| unsafe { w.bits(0) });

			// Start SPI transaction.
			spim.tasks_start.write(|w| unsafe { w.bits(1) });
	
			// Wait for END event - triggered when tx and rx are done
			while spim.events_end.read().bits() == 0 {}
	
			// Clear the end flag
			spim.events_end.write(|w| unsafe { w.bits(0) });
	
			if spim.txd.amount.read().bits() != len {
				return Err(SpimError::Transmit);
			}
			if spim.rxd.amount.read().bits() != len {
				return Err(SpimError::Receive);
			}

			spim.enable.write(|w| w.enable().disabled());
			gpio::set_pin_state(config::SPI_SCLK_PIN, gpio::PinState::PinHigh);

			Ok(())
		}
		else {
			Err(SpimError::Handler)
		}
	})
}

//==============================================================================
// Private Functions
//==============================================================================
fn configure(spim: &nrf52832_pac::SPIM0) {
	spim.enable.write(|w| w.enable().disabled());

	// Configure SCLK pin
	gpio::pin_setup(SPIM_LINE.sclk_pin, DIR::OUTPUT, gpio::PinState::PinLow, PULL::DISABLED);
	spim.psel.sck.write(|w| unsafe { w.bits(SPIM_LINE.sclk_pin as u32) });

	// Configure MISO pin
	gpio::pin_setup(SPIM_LINE.miso_pin, DIR::INPUT, gpio::PinState::PinHigh, PULL::PULLUP);
	spim.psel.miso.write(|w| unsafe { w.bits(SPIM_LINE.miso_pin as u32) });

	// Configure MOSI pin
	gpio::pin_setup(SPIM_LINE.mosi_pin, DIR::OUTPUT, gpio::PinState::PinLow, PULL::DISABLED);
	spim.psel.mosi.write(|w| unsafe { w.bits(SPIM_LINE.mosi_pin as u32) });

	spim.frequency.write(|w| w.frequency().variant(SPIM_LINE.frequency));
	spim.config.write(|w| w
		.order().variant(SPIM_LINE.order)
		.cpha().variant(SPIM_LINE.cpha)
		.cpol().variant(SPIM_LINE.cpol)
	);

	// Ensure we are using the ArrayList structure
	spim.rxd.list.write(|w| w.list().variant(nrf52832_pac::spim0::rxd::list::LIST_A::ARRAYLIST));
	spim.txd.list.write(|w| w.list().variant(nrf52832_pac::spim0::txd::list::LIST_A::ARRAYLIST));
}

fn get_tx_ptr() -> u32 {
	static mut LAST_PTR: bool = true;
	unsafe { 
		if LAST_PTR {
			LAST_PTR = false;
			config::SPIM_DMA_TX_PTR_A
		}
		else {
			LAST_PTR = false;
			config::SPIM_DMA_TX_PTR_B
		}
	}
}

fn is_block_valid(ptr: u32) -> bool {
	if (ptr >= config::SPIM_DMA_MIN) && 
		(ptr < (config::SPIM_DMA_MAX - config::SPIM_DMA_SIZE)) {
		true
	}
	else {
		false
	}
}

fn make_ram(block: &[u8]) -> (u32, u32) {
	let len = block.len();

	if !is_block_valid(block.as_ptr() as u32) {
		let ptr = get_tx_ptr() as usize;
		unsafe { for i in 0..len {
			ptr::write((ptr+i) as *mut u8, block[i]);
		} }
		(ptr as u32, len as u32)
	}
	else {
		(block.as_ptr() as u32, len as u32)
	}
}

//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
