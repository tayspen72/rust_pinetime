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
	HANDLER,
	RECEIVE,
	STOP,
	TRANSMIT
}
//==============================================================================
// Variables
//==============================================================================
static I2C_HANDLE: Mutex<RefCell<Option<nrf52832_pac::TWI1>>> = 
	Mutex::new(RefCell::new(None));

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
pub fn read(address: u8, buffer: &mut [u8]) -> Result<(), I2cError> {
	free(|cs| {
		if let Some(ref mut i2c) = I2C_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			// Disable all shorcuts
			i2c.shorts.write(|w| w
				.bb_stop().disabled()
				.bb_suspend().disabled()
			);

			// Make sure the right address is being used
			i2c.address.write(|w| unsafe { w.address().bits(address.into()) });
			
			// Split the transaction into the bulk of the read and the last byte to be read
			if let Some((last, data)) = buffer.split_last_mut() {
				if !data.is_empty() {
					i2c.shorts.write(|w| w.bb_suspend().enabled());
				}
				else{
					i2c.shorts.write(|w| w.bb_stop().enabled());
				}

				// Clear flag showing new data in register
				i2c.events_rxdready.write(|w| unsafe { w.bits(0) });

				// Start the first Rx task
				i2c.tasks_startrx.write(|w| unsafe { w.bits(1) } );

				for byte in &mut data.into_iter() {
					i2c.tasks_resume.write(|w| unsafe { w.bits(1) });
					*byte = rx_byte(i2c)?;
				}
				
				i2c.shorts.write(|w| w.bb_stop().enabled());
				i2c.tasks_resume.write(|w| unsafe { w.bits(1) });
				*last = rx_byte(i2c)?;
			}
			else{
				send_stop(i2c)?;
			}
			
			Ok(())
		}
		else {
			Err(I2cError::HANDLER)
		}
	})
}

#[allow(dead_code)]
pub fn write(address: u8, data: &[u8]) -> Result<(), I2cError> {
	free(|cs| {
		if let Some(ref mut i2c) = I2C_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			// Disable all shorcuts
			i2c.shorts.write(|w| w
				.bb_stop().disabled()
				.bb_suspend().disabled()
			);

			// Make sure the right address is being used
			i2c.address.write(|w| unsafe { w.address().bits(address.into()) });

			// Start the transaction
			i2c.tasks_starttx.write(|w| unsafe { w.bits(1) });

			for byte in data.into_iter() {
				tx_byte(i2c, *byte)?;
			}
			
			send_stop(i2c)?;
			Ok(())
		}
		else {
			Err(I2cError::HANDLER)
		}
	})
}

pub fn write_then_read(address: u8, tx_buffer: &[u8], rx_buffer: &mut [u8]) -> Result<(), I2cError> {
	free(|cs| {
		if let Some(ref mut i2c) = I2C_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			// Disable all shorcuts
			i2c.shorts.write(|w| w
				.bb_stop().disabled()
				.bb_suspend().disabled()
			);

			// Make sure the right address is being used
			i2c.address.write(|w| unsafe { w.address().bits(address.into()) });

			// Start the transaction
			i2c.tasks_starttx.write(|w| unsafe { w.bits(1) });

			// Send out all tx buffer contents
			for byte in tx_buffer {
				tx_byte(i2c, *byte)?;
			}

			if let Some((last, data)) = rx_buffer.split_last_mut() {
				if !data.is_empty() {
					i2c.shorts.write(|w| w.bb_suspend().enabled());
				}
				else{
					i2c.shorts.write(|w| w.bb_stop().enabled());
				}

				// Clear flag showing new data in register
				i2c.events_rxdready.write(|w| unsafe { w.bits(0) });

				// Start the first Rx task
				i2c.tasks_startrx.write(|w| unsafe { w.bits(1) } );

				for byte in &mut data.into_iter() {
					i2c.tasks_resume.write(|w| unsafe { w.bits(1) });
					*byte = rx_byte(i2c)?;
				}
				
				i2c.shorts.write(|w| w.bb_stop().enabled());
				i2c.tasks_resume.write(|w| unsafe { w.bits(1) });
				*last = rx_byte(i2c)?;
			}
			else{
				send_stop(i2c)?;
			}

			Ok(())
		}
			else {
				Err(I2cError::HANDLER)
			}
		})
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

fn rx_byte(i2c: &nrf52832_pac::TWI1) -> Result<u8, I2cError> {
	// Wait for rx event or error out
	while (i2c.events_rxdready.read().bits() == 0) && 
		(i2c.events_error.read().bits() == 0) {}
	
	// If error, bail out
	if i2c.events_error.read().bits() > 0 {
		i2c.events_error.write(|w| unsafe { w.bits(0) });
		return Err(I2cError::RECEIVE);
	}
	
	let byte = i2c.rxd.read().rxd().bits();

	// Clear out the Rx event flag
	i2c.events_rxdready.write(|w| unsafe { w.bits(0) });
	
	Ok(byte)
}

fn tx_byte(i2c: &nrf52832_pac::TWI1, byte: u8) -> Result<(), I2cError> {
	// Wait for rx event or error out
	i2c.events_txdsent.write(|w| unsafe { w.bits(0) });

	// Load in the byte to be sent
	i2c.txd.write(|w| unsafe { w.bits(byte.into()) });
	
	// Wait until transmission is completed
	while (i2c.events_txdsent.read().bits() == 0) && 
		(i2c.events_error.read().bits() == 0) {}
		
	// If error, bail out
	if i2c.events_error.read().bits() > 0 {
		i2c.events_error.write(|w| unsafe { w.bits(0) });
		return Err(I2cError::TRANSMIT);
	}
	
	// Clear out the Rx event flag
	i2c.events_txdsent.write(|w| unsafe { w.bits(0) });
	
	Ok(())
}

fn send_stop(i2c: &nrf52832_pac::TWI1) -> Result<(), I2cError> {
	// Clear stopped event.
	i2c.events_stopped.write(|w| unsafe { w.bits(0) });

	// Start stop condition.
	i2c.tasks_stop.write(|w| unsafe { w.bits(1) });

	// Wait until stop was sent.
	while i2c.events_stopped.read().bits() == 0 &&
		(i2c.events_error.read().bits() == 0) {}

	// Bail out if we get an error instead.
	if i2c.events_error.read().bits() != 0 {
		i2c.events_error.write(|w| unsafe { w.bits(0) });
		return Err(I2cError::STOP);
	}
	
	Ok(())
}

//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
