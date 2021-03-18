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
use nrf52832_pac::spim0;
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
	pub frequency: spim0::frequency::FREQUENCY_A,
	pub order: spim0::config::ORDER_A,
	pub cpha: spim0::config::CPHA_A,
	pub cpol: spim0::config::CPOL_A
}

type ArrayList = [u8];

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

static SPIM_HANDLE: Mutex<RefCell<Option<nrf52832_pac::SPIM0>>> = 
	Mutex::new(RefCell::new(None));

//==============================================================================
// Public Functions
//==============================================================================
pub fn init(spim0: nrf52832_pac::SPIM0) {
	configure(&spim0);

	free(|cs| SPIM_HANDLE.borrow(cs).replace(Some(spim0)));
}

pub fn dma_cleanup() {
	free(|cs| {
		if let Some(ref mut spim) = SPIM_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			spim.tasks_stop.write(|w| unsafe { w.bits(1) });

			spim.events_stopped.write(|w| unsafe { w.bits(0) });
			spim.events_endrx.write(|w| unsafe { w.bits(0) });
			spim.events_end.write(|w| unsafe { w.bits(0) });
			spim.events_endtx.write(|w| unsafe { w.bits(0) });
			spim.events_started.write(|w| unsafe { w.bits(0) });
		}
	});
}

pub fn get_busy_dma() -> bool {
	free(|cs| {
		if let Some(ref mut spim) = SPIM_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			(spim.events_started.read().bits() != 0) && 
			(spim.events_endtx.read().bits() == 0)
		}
		else {
			false
		}
	})
}

pub fn setup_block(block: &ArrayList){
	// Pull ptrs to the open RAM banks
	let (rx_ptr, tx_ptr): (usize, usize) = get_open_spim_bank();
	let len = if block.len() > 256 { 256 } else { block.len() };
	// Toggle the active bank
	toggle_spim_bank();

	for i in 0..len {
		unsafe { 
			ptr::write((tx_ptr+i) as *mut u8, block[i]);
		}
	}

	free(|cs| {
		if let Some(ref mut spim) = SPIM_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			spim.rxd.maxcnt.write(|w| unsafe { w.maxcnt().bits(len as u8) });
			spim.rxd.ptr.write(|w| unsafe { w.ptr().bits(rx_ptr as u32) });
			spim.txd.maxcnt.write(|w| unsafe { w.maxcnt().bits(len as u8) });
			spim.txd.ptr.write(|w| unsafe { w.ptr().bits(tx_ptr as u32) });
		}
	});
}

pub fn start_block() {
	while get_busy_dma() {}
	dma_cleanup();

	free(|cs| {
		if let Some(ref mut spim) = SPIM_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			spim.tasks_start.write(|w| unsafe { w.bits(1) });
		}
	});
}

pub fn tx_data(data: &ArrayList) {
	// Initialize the transmit registers with the data payload
	setup_block(data);

	// Start the transfer when ready
	start_block();

	// Wait for transfer to complete
	while get_busy_dma() {};

	// Reset the event registers when complete
	dma_cleanup();
}

//==============================================================================
// Private Functions
//==============================================================================
fn configure(spim: &nrf52832_pac::SPIM0) {
	spim.enable.write(|w| w.enable().disabled());

	// Configure MOSI pin
	gpio::pin_setup(SPI_LINE.mosi_pin, DIR::OUTPUT, gpio::PinState::PinLow, PULL::DISABLED);
	spim.psel.mosi.write(|w| unsafe { w.bits(SPI_LINE.mosi_pin as u32) });

	// Configure MISO pin
	gpio::pin_setup(SPI_LINE.miso_pin, DIR::INPUT, gpio::PinState::PinHigh, PULL::PULLUP);
	spim.psel.miso.write(|w| unsafe { w.bits(SPI_LINE.miso_pin as u32) });

	// Configure SCLK pin
	gpio::pin_setup(SPI_LINE.sclk_pin, DIR::OUTPUT, gpio::PinState::PinLow, PULL::DISABLED);
	spim.psel.sck.write(|w| unsafe { w.bits(SPI_LINE.sclk_pin as u32) });

	spim.frequency.write(|w| w.frequency().variant(SPI_LINE.frequency));
	spim.config.write(|w| w
		.order().variant(SPI_LINE.order)
		.cpha().variant(SPI_LINE.cpha)
		.cpol().variant(SPI_LINE.cpol)
	);

	// Ensure we are using the ArrayList structure
	spim.rxd.list.write(|w| w.list().variant(nrf52832_pac::spim0::rxd::list::LIST_A::ARRAYLIST));
	spim.txd.list.write(|w| w.list().variant(nrf52832_pac::spim0::txd::list::LIST_A::ARRAYLIST));

	spim.enable.write(|w| w.enable().enabled());
}

fn get_open_spim_bank() -> (usize, usize) {
	match free(|cs| SPIM_ACTIVE_BANK.borrow(cs).get()) {
		ActiveBank::BankA => (config::SPIM_RX_BANKB, config::SPIM_TX_BANKB),
		ActiveBank::BankB => (config::SPIM_RX_BANKA, config::SPIM_TX_BANKA),
	}
}

fn toggle_spim_bank() {
	free(|cs| SPIM_ACTIVE_BANK.borrow(cs).set(
		if let ActiveBank::BankA = SPIM_ACTIVE_BANK.borrow(cs).get() {
			ActiveBank::BankB
		}
		else {
			ActiveBank::BankA
		}
	));
}

//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
