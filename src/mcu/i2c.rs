//==============================================================================
// Notes
//==============================================================================
// mcu::i2c.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use core::cell::RefCell;
use core::ops::DerefMut;
use cortex_m::interrupt::{free, Mutex};
use crate::config;
use nrf52832_pac;
use super::gpio;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[derive(Copy, Clone)]
pub struct I2cLine {
	pub scl_pin: u8,
	pub sda_pin: u8,
	pub frequency: nrf52832_pac::twi0::frequency::FREQUENCY_A,
}

pub enum I2cError {
	OVERRUN,
	ANACK,
	DNACK,
	HANDLER,
	UNKNOWN
}
//==============================================================================
// Variables
//==============================================================================
static I2C_HANDLE: Mutex<RefCell<Option<nrf52832_pac::TWI1>>> = 
	Mutex::new(RefCell::new(None));

const RX_BUFFER_LENGTH: usize = 63;
static mut RX_BUFFER: [u8; RX_BUFFER_LENGTH] = [0; RX_BUFFER_LENGTH];
static mut HEAD: usize = 0;
static mut TAIL: usize = 0;

const I2C_LINE: I2cLine = I2cLine {
	scl_pin: config::I2C_SCL_PIN,
	sda_pin: config::I2C_SDA_PIN,
	frequency: config::I2C_FREQUENCY,
};

//==============================================================================
// Public Functions
//==============================================================================
pub fn init(i2c: nrf52832_pac::TWI1){
	configure(&i2c);

	free(|cs| I2C_HANDLE.borrow(cs).replace(Some(i2c)));
}

pub fn pop_byte() -> u8 {
	unsafe {
		let byte: u8  = RX_BUFFER[HEAD];
		HEAD = HEAD + 1;
		if HEAD >= RX_BUFFER_LENGTH {
			HEAD = 0; 
		}
		byte
	}
}

#[allow(dead_code)]
pub fn read_byte(address: u8, send_stop: bool) -> Option<I2cError> {
	set_address(address);

	free(|cs| {
		if let Some(ref mut i2c) = I2C_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			// Start Rx task
			i2c.tasks_startrx.write(|w| unsafe { w.bits(1) } );
			
			// Wait for rx event or error out
			while (i2c.events_rxdready.read().bits() == 0) && 
				(i2c.events_error.read().bits() == 0) {}
			
			// If error, bail out
			if i2c.events_error.read().bits() > 0 {
				i2c.events_error.write(|w| unsafe { w.bits(0) });
				i2c.tasks_stop.write(|w| unsafe { w.bits(1) } );
				let error = get_errorsrc(i2c.errorsrc.read().bits());
				return Some(error);
			}
			
			// Send stop before reading rxd as it could initiate another rx when read
			if send_stop {
				i2c.tasks_stop.write(|w| unsafe { w.bits(1) } );
			}
			
			// Clear out the Rx event flag
			i2c.events_rxdready.write(|w| unsafe { w.bits(0) });
			
			// Push byte into rx buffer
			unsafe {
				RX_BUFFER[TAIL] = i2c.rxd.read().rxd().bits();
				TAIL = TAIL + 1;
				if TAIL >= RX_BUFFER.len() {
					TAIL = 0; 
				}
			}

			// Cleanup when finished, just to be safe
			i2c.events_error.write(|w| unsafe { w.bits(0) });
			i2c.events_rxdready.write(|w| unsafe { w.bits(0) });
			
			None
		}
		else{
			Some(I2cError::HANDLER)
		}
	})
}

#[allow(dead_code)]
pub fn read_data(address: u8, send_stop: bool, len: usize) -> Option<I2cError> {
	set_address(address);
	
	free(|cs| {
		if let Some(ref mut i2c) = I2C_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			// Start the first Rx task
			i2c.tasks_startrx.write(|w| unsafe { w.bits(1) } );

			for i in 0..len {
				// Wait for rx event or error out
				while (i2c.events_rxdready.read().bits() == 0) && 
					(i2c.events_error.read().bits() == 0) {}
				
				// If error, bail out
				if i2c.events_error.read().bits() > 0 {
					i2c.events_error.write(|w| unsafe { w.bits(0) });
					i2c.tasks_stop.write(|w| unsafe { w.bits(1) } );
					let error = get_errorsrc(i2c.errorsrc.read().bits());
					return Some(error);
				}
				
				// Push byte into rx buffer
				unsafe {
					RX_BUFFER[TAIL] = i2c.rxd.read().rxd().bits();
					TAIL = TAIL + 1;
					if TAIL >= RX_BUFFER_LENGTH {
						TAIL = 0;
					}
				}
				
				// Clear flag showing new data in register
				i2c.events_rxdready.write(|w| unsafe { w.bits(0) });

				// If not end, resume
				if i < (len - 1) {
					i2c.tasks_resume.write(|w| unsafe { w.bits(1) });
				}
				else if send_stop {
					i2c.tasks_stop.write(|w| unsafe { w.bits(1) } );
				}
			}
			
			// Cleanup when finished, just to be safe
			i2c.events_error.write(|w| unsafe { w.bits(0) });
			i2c.events_rxdready.write(|w| unsafe { w.bits(0) });
			i2c.rxd.read().rxd().bits();

			return None
		}
		else {
			return Some(I2cError::HANDLER)
		}
	})
}

#[allow(dead_code)]
pub fn write_byte(address: u8, byte: u8, send_start_command: bool, send_stop_command: bool) -> Option<I2cError> {	
	set_address(address);
	
	if send_start_command {
		send_start();
	}

	free(|cs| {
		if let Some(ref mut i2c) = I2C_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			// Preload the Txd register for sending
			i2c.txd.write(|w| unsafe { w.txd().bits(byte) } );
			
			// Wait for rx event or error out
			while (i2c.events_txdsent.read().bits() == 0) && 
				(i2c.events_error.read().bits() == 0) {}

			// If error, bail out
			if i2c.events_error.read().bits() > 0 {
				i2c.events_error.write(|w| unsafe { w.bits(0) });
				i2c.tasks_stop.write(|w| unsafe { w.bits(1) } );
				let error = get_errorsrc(i2c.errorsrc.read().bits());
				return Some(error);
			}
			
			if send_stop_command {
				i2c.tasks_stop.write(|w| unsafe { w.bits(1) } );
			}
			
			// Clear out the Tx event flag
			i2c.events_txdsent.write(|w| unsafe { w.bits(0) });
			None
		}
		else {
			Some(I2cError::HANDLER)
		}
	})
}

#[allow(dead_code)]
pub fn write_data(address: u8, data: &[u8], send_start_command: bool, send_stop_command: bool) -> Option<I2cError> {
	if send_start_command {
		send_start();
	}
	
	for i in 0..data.len() {
		match write_byte(address, data[i], false, send_stop_command && (i == (data.len() - 1))) {
			Some(e) => return Some(e),
			None => ()
		};
	}
	
	None
}

//==============================================================================
// Private Functions
//==============================================================================
fn configure(i2c: &nrf52832_pac::TWI1) {
	i2c.enable.write(|w| w.enable().disabled());
	
	gpio::pin_setup(
		I2C_LINE.scl_pin,
		nrf52832_pac::p0::pin_cnf::DIR_A::INPUT,
		gpio::PinState::PinHigh,
		nrf52832_pac::p0::pin_cnf::PULL_A::DISABLED);
	i2c.pselscl.write(|w| unsafe { w.bits( I2C_LINE.scl_pin as u32) });

	gpio::pin_setup(
		I2C_LINE.sda_pin,
		nrf52832_pac::p0::pin_cnf::DIR_A::INPUT,
		gpio::PinState::PinHigh,
		nrf52832_pac::p0::pin_cnf::PULL_A::DISABLED);
	i2c.pselsda.write(|w| unsafe { w.bits( I2C_LINE.sda_pin as u32) });

	i2c.frequency.write(|w| w.frequency().variant( I2C_LINE.frequency));
	
	i2c.enable.write(|w| w.enable().enabled());
}

#[inline(always)]
fn get_errorsrc(val: u32) -> I2cError {
	match val {
		1 => I2cError::OVERRUN,
		2 => I2cError::ANACK,
		4 => I2cError::DNACK,
		_ => I2cError::UNKNOWN,
	}
}

fn send_start() {
	free(|cs| {
		if let Some(ref mut i2c) = I2C_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			i2c.tasks_starttx.write(|w| unsafe { w.bits(1) } );
		}
	});
}

fn set_address(address: u8) {
	static mut CURRENT_ADDRESS: u8 = 0;

	if address == unsafe { CURRENT_ADDRESS } {
		return;
	}

	free(|cs| {
		if let Some(ref mut i2c) = I2C_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			if i2c.address.read().bits() as u8 == address {
				return;
			}

			i2c.address.write(|w| unsafe { w.address().bits(address.into()) } );
		}
	});
	
	unsafe { CURRENT_ADDRESS = address };
}

//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
