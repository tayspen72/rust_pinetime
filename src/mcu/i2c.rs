//==============================================================================
// Notes
//==============================================================================
// mcu::i2c.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use nrf52832_pac;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[allow(dead_code)]
pub struct I2cLine {
	pub scl_pin: u8,
	pub sda_pin: u8,
	pub frequency: nrf52832_pac::twi0::frequency::FREQUENCY_A,
	pub address: u8,
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
pub fn init(p: &nrf52832_pac::Peripherals, i2cline: &I2cLine){
	let i2c = &p.TWI0;
	
	i2c.enable.write(|w| w.enable().disabled());
	
	i2c.pselscl.write(|w| unsafe { w.bits(i2cline.scl_pin as u32) } );
	i2c.pselsda.write(|w| unsafe { w.bits(i2cline.sda_pin as u32) } );
	i2c.frequency.write(|w| w.frequency().variant(i2cline.frequency));
	i2c.address.write(|w| unsafe { w.address().bits(i2cline.address) } );
	
	i2c.enable.write(|w| w.enable().enabled());
}

#[allow(dead_code)]
pub fn write_byte(p: &nrf52832_pac::Peripherals, byte: u8, send_start: bool, send_stop: bool) -> Option<bool>{	
	let i2c = &p.TWI0;
	
	// Preload the Txd register for sending
	i2c.txd.write(|w| unsafe { w.txd().bits(byte) } );
	
	if send_start {
		i2c.tasks_starttx.write(|w| unsafe { w.bits(1) } );
	}
	
	// Wait for rx event or error out
	while i2c.events_txdsent.read().bits() == i2c.events_error.read().bits() {}
	
	// If error, bail out
	if i2c.events_error.read().bits() > 0 {
		i2c.events_error.write(|w| unsafe { w.bits(0) });
		i2c.tasks_stop.write(|w| unsafe { w.bits(1) } );
		return None;
	}
	
	if send_stop {
		i2c.tasks_stop.write(|w| unsafe { w.bits(1) } );
	}
	
	// Clear out the Tx event flag
	i2c.events_txdsent.write(|w| unsafe { w.bits(0) });
	
	Some(true)
}

#[allow(dead_code)]
pub fn write_data(p: &nrf52832_pac::Peripherals, data: &[u8], send_start: bool, send_stop: bool) -> Option<bool> {
	if send_start {
		p.TWI0.tasks_starttx.write(|w| unsafe { w.bits(1) } );
	}
	
	for i in 0..data.len() {
		match write_byte(p, data[i], false, send_stop && (i == (data.len() - 1))) {
			None => return None,
			_ => ()
		};
	}
	
	Some(true)
}

#[allow(dead_code)]
pub fn read_byte(p: &nrf52832_pac::Peripherals, send_stop: bool) -> Option<u8> {
	let i2c = &p.TWI0;
	
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
	
	// Pull out byte
	Some(i2c.rxd.read().rxd().bits())
}

//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
