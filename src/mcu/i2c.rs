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

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[derive(Copy, Clone)]
pub struct I2cLine {
	pub scl_pin: u8,
	pub sda_pin: u8,
	pub frequency: nrf52832_pac::twi0::frequency::FREQUENCY_A,
}

//==============================================================================
// Variables
//==============================================================================
static I2C_HANDLE: Mutex<RefCell<Option<nrf52832_pac::TWI1>>> = 
	Mutex::new(RefCell::new(None));

const RX_BUFFER_LENGTH: usize = 64;
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

#[allow(dead_code)]
pub fn write_byte(address: u8, byte: u8, send_start_command: bool, send_stop_command: bool) -> Option<bool> {	
	set_address(address);
	
	if send_start_command {
		send_start();
	}

	free(|cs| {
		if let Some(ref mut i2c) = I2C_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			// Preload the Txd register for sending
			i2c.txd.write(|w| unsafe { w.txd().bits(byte) } );
			
			// Wait for rx event or error out
			while i2c.events_txdsent.read().bits() == i2c.events_error.read().bits() {}
			
			// If error, bail out
			if i2c.events_error.read().bits() > 0 {
				i2c.events_error.write(|w| unsafe { w.bits(0) });
				i2c.tasks_stop.write(|w| unsafe { w.bits(1) } );
				return Some(false);
			}
			
			if send_stop_command {
				i2c.tasks_stop.write(|w| unsafe { w.bits(1) } );
			}
			
			// Clear out the Tx event flag
			i2c.events_txdsent.write(|w| unsafe { w.bits(0) });
			Some(true)
		}
		else {
			None
		}
	})
}

#[allow(dead_code)]
pub fn write_data(address: u8, data: &[u8], send_start_command: bool, send_stop_command: bool) -> Option<bool> {
	if send_start_command {
		send_start();
	}
	
	for i in 0..data.len() {
		match write_byte(address, data[i], false, send_stop_command && (i == (data.len() - 1))) {
			None => return None,
			Some(false) => return Some(false),
			Some(true) => ()
		};
	}
	
	Some(true)
}

pub fn pop_byte() -> u8 {
	unsafe {
		let byte: u8  = RX_BUFFER[TAIL];
		HEAD = HEAD + 1;
		if HEAD >= RX_BUFFER.len() {
			HEAD = 0; 
		}
		byte
	}
}

#[allow(dead_code)]
pub fn read_byte(address: u8, send_stop: bool) -> Option<bool> {
	set_address(address);

	free(|cs| {
		if let Some(ref mut i2c) = I2C_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			// Start Rx task
			i2c.tasks_startrx.write(|w| unsafe { w.bits(1) } );
			
			// Wait for rx event or error out
			while i2c.events_rxdready.read().bits() == i2c.events_error.read().bits() {}
			
			// If error, bail out
			if i2c.events_error.read().bits() > 0 {
				i2c.events_error.write(|w| unsafe { w.bits(0) });
				i2c.tasks_stop.write(|w| unsafe { w.bits(1) } );
				return None;
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
			Some(true)
		}
		else{
			None
		}
	})
}

#[allow(dead_code)]
pub fn read_data(address: u8, send_stop: bool, len: u16) {
	set_address(address);

	free(|cs| {
		if let Some(ref mut i2c) = I2C_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			for _ in 0..len {
				// Start Rx task
				i2c.tasks_startrx.write(|w| unsafe { w.bits(1) } );
				
				// Wait for rx event or error out
				while i2c.events_rxdready.read().bits() == i2c.events_error.read().bits() {}
				
				// If error, bail out
				if i2c.events_error.read().bits() > 0 {
					i2c.events_error.write(|w| unsafe { w.bits(0) });
					i2c.tasks_stop.write(|w| unsafe { w.bits(1) } );
					return;
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
			}
		}
	})
}

//==============================================================================
// Private Functions
//==============================================================================
fn configure(i2c: &nrf52832_pac::TWI1) {
	i2c.enable.write(|w| w.enable().disabled());
	
	i2c.pselscl.write(|w| unsafe { w.bits( I2C_LINE.scl_pin as u32) });
	i2c.pselsda.write(|w| unsafe { w.bits( I2C_LINE.sda_pin as u32) });
	i2c.frequency.write(|w| w.frequency().variant( I2C_LINE.frequency));
	
	i2c.enable.write(|w| w.enable().enabled());
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

			i2c.address.write(|w| unsafe { w.address().bits(address) } );
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
