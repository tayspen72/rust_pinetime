//==============================================================================
// Notes
//==============================================================================
// drivers::lcd::lcd.rs
// LCD Essential Functions

//==============================================================================
// Crates and Mods
//==============================================================================
use heapless::Vec;
use crate::config;
use crate::drivers::log;
use crate::mcu::{gpio, spi, spim, timer};
use nrf52832_pac::p0::pin_cnf::DIR_A as DIR;
use nrf52832_pac::p0::pin_cnf::PULL_A as PULL;

use super::st7789;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================


//==============================================================================
// Variables
//==============================================================================


//==============================================================================
// Public Functions
//==============================================================================
pub fn init() {
	// Initialize lcd control pins
	gpio::pin_setup(config::LCD_CS_PIN, DIR::OUTPUT, gpio::PinState::PinHigh, PULL::DISABLED);
	gpio::pin_setup(config::LCD_DCX_PIN, DIR::OUTPUT, gpio::PinState::PinHigh, PULL::DISABLED);
	gpio::pin_setup(config::LCD_RESET_PIN, DIR::OUTPUT, gpio::PinState::PinLow, PULL::PULLUP);

	configure();

	gpio::pin_setup(config::LCD_BACKLIGHT_LOW, DIR::OUTPUT, gpio::PinState::PinHigh, PULL::DISABLED);
	gpio::pin_setup(config::LCD_BACKLIGHT_MID, DIR::OUTPUT, gpio::PinState::PinHigh, PULL::DISABLED);
	gpio::pin_setup(config::LCD_BACKLIGHT_HIGH, DIR::OUTPUT, gpio::PinState::PinHigh, PULL::DISABLED);
	set_backlight(0);
}

pub fn set_backlight(backlight: u8) {
	let mut states: [gpio::PinState; 3] = [gpio::PinState::PinHigh; 3];
	for s in 0..3 {
		states[s] = {
			if backlight & (1 << s) == 0 {
				gpio::PinState::PinHigh }
			else {
				gpio::PinState::PinLow
			}
		};
	}

	gpio::set_pin_state(config::LCD_BACKLIGHT_LOW, states[0]);
	gpio::set_pin_state(config::LCD_BACKLIGHT_MID, states[1]);
	gpio::set_pin_state(config::LCD_BACKLIGHT_HIGH, states[2]);
}

pub fn write_command(command: st7789::COMMAND) {
	gpio::set_pin_state(config::LCD_CS_PIN, gpio::PinState::PinLow);
	gpio::set_pin_state(config::LCD_DCX_PIN, gpio::PinState::PinLow);

	if let Err(_e) = spi::write(&[command as u8]) {
		log::push_log("Spi command failed");
	}

	gpio::set_pin_state(config::LCD_DCX_PIN, gpio::PinState::PinHigh);
	gpio::set_pin_state(config::LCD_CS_PIN, gpio::PinState::PinHigh);
}

pub fn write_data(data: &[u8]) {
	gpio::set_pin_state(config::LCD_CS_PIN, gpio::PinState::PinLow);
	gpio::set_pin_state(config::LCD_DCX_PIN, gpio::PinState::PinHigh);

	if let Err(_e) = spi::write(data) {
		log::push_log("Spi write data failed");
	}

	gpio::set_pin_state(config::LCD_CS_PIN, gpio::PinState::PinHigh);
}

pub fn write_block(data: &[u8]) {
	gpio::set_pin_state(config::LCD_CS_PIN, gpio::PinState::PinLow);
	gpio::set_pin_state(config::LCD_DCX_PIN, gpio::PinState::PinHigh);

	let mut bytes_remaining = data.len();
	let mut current_index: usize = 0;
	let mut v: Vec<u8, 255> = Vec::new();

	while bytes_remaining > 0 {
		v.clear();
		let bytes_this_transfer = if bytes_remaining > 0xFF {
			0xFF
		}
		else {
			bytes_remaining
		};

		if let Ok(()) = v.extend_from_slice(
			&data[current_index..(current_index + bytes_this_transfer)]
		) {
			if let Err(_e) = spim::write(&v[..bytes_this_transfer]) {
				log::push_log("Spim write block failed");
				break;
			}
		}
		else {
			break;
		}

		current_index += bytes_this_transfer;
		bytes_remaining -= bytes_this_transfer;
	}
	
	gpio::set_pin_state(config::LCD_CS_PIN, gpio::PinState::PinHigh);
}

pub fn write_block_solid(color: u16, len: u32) {
	gpio::set_pin_state(config::LCD_CS_PIN, gpio::PinState::PinLow);
	gpio::set_pin_state(config::LCD_DCX_PIN, gpio::PinState::PinHigh);

	let block: [u16; 127] = [color; 127];
	let block: [u8; 254] = unsafe {
		core::mem::transmute::<[u16; 127], [u8; 254]>(block)
	};
	let mut v: Vec<u8, 254> = Vec::new();
	let mut remaining = len;
	if let Ok(()) = v.extend_from_slice(&block[..]) {
		while remaining > 0 {
			if remaining > 127 {
				if let Err(_e) = spim::write(&v[..]) {
					log::push_log("Spim write solid failed");
				}
				remaining -= 127;
			}
			else {
				let end: usize = (remaining * 2) as usize;
				if let Err(_e) = spim::write(&v[0..end]) {
					log::push_log("Spim write solid failed");
				}
				remaining = 0;
			}
		}
	}
	else {
		log::push_log("Vec extend failed");
	}

	gpio::set_pin_state(config::LCD_CS_PIN, gpio::PinState::PinHigh);
}

//==============================================================================
// Private Functions
//==============================================================================
fn configure() {
	// Enter safe reset sequence
	gpio::set_pin_state(config::LCD_RESET_PIN, gpio::PinState::PinHigh);
	timer::delay(5);
	gpio::set_pin_state(config::LCD_RESET_PIN, gpio::PinState::PinLow);
	timer::delay(20);
	gpio::set_pin_state(config::LCD_RESET_PIN, gpio::PinState::PinHigh);
	timer::delay(150);

	// Also initiate a software reset - just to be safe
	write_command(st7789::COMMAND::SW_RESET);
	timer::delay(150);

	// Exit sleep
	write_command(st7789::COMMAND::SLEEP_OUT);
	timer::delay(150);

	write_command(st7789::COMMAND::NORMAL_MODE);
	
	// Write memory data format: 
	//  RGB, left to right, top to bottom, logical direction of memory pointer updates
	write_command(st7789::COMMAND::MEMORY_DATA_ACCESS_CONTROL);
	write_data(&mut [ 0x08 ]);

	// Define pixel interfacing format:
	//  5-6-5 for 65k color options
	write_command(st7789::COMMAND::INTERFACE_PIXEL_FORMAT);
	write_data(&mut [ 0x55 ]);
	timer::delay(10);

	write_command(st7789::COMMAND::PORCH_SETTING);
	write_data(&mut [ 0x0c, 0x0c, 0x00, 0x33, 0x33 ]);

	write_command(st7789::COMMAND::GATE_CONTROL);
	write_data(&mut [ 0x35 ]);

	write_command(st7789::COMMAND::GATE_ON_TIMING_ADJUSTMENT);
	write_data(&mut [ 0x28 ]);

	write_command(st7789::COMMAND::LCM_CONTROL);
	write_data(&mut [ 0x0C ]);

	write_command(st7789::COMMAND::VDV_VRH_CMD_ENABLE);
	write_data(&mut [ 0x01, 0xFF ]);

	write_command(st7789::COMMAND::VRH_SET);
	write_data(&mut [ 0x01 ]);

	write_command(st7789::COMMAND::VDV_SET);
	write_data(&mut [ 0x20 ]);

	write_command(st7789::COMMAND::FRAME_RATE_CONTROL_2);
	write_data(&mut [ 0x0F ]);

	write_command(st7789::COMMAND::POWER_CONTROL_1);
	write_data(&mut [ 0xA4, 0xA1 ]);

	write_command(st7789::COMMAND::POSITIVE_VOLTAGE_GAMMA_CONTROL);
	write_data(&mut [ 0xd0, 0x00, 0x02, 0x07, 0x0a, 0x28, 0x32, 0x44, 0x42, 0x06, 0x0e, 0x12, 0x14, 0x17 ]);
	
	write_command(st7789::COMMAND::NEGATIVE_VOLTAGE_GAMMA_CONTROL);
	write_data(&mut [ 0xd0, 0x00, 0x02, 0x07, 0x0a, 0x28, 0x31, 0x54, 0x47, 0x0e, 0x1c, 0x17, 0x1b, 0x1e ]); 	
	
	write_command(st7789::COMMAND::DISPLAY_INVERSION_ON);
	
	write_command(st7789::COMMAND::DISPLAY_BRIGHTNESS);
	write_data(&mut [ 0x7F ]);	//initial 25% brightness

	write_command(st7789::COMMAND::GAMMA);
	write_data(&mut [ 0x04 ]);

	// Explicitly end this bulk write
	gpio::set_pin_state(config::LCD_CS_PIN, gpio::PinState::PinHigh);

	timer::delay(120);
	
	write_command(st7789::COMMAND::DISPLAY_ON);
	
	timer::delay(120);
}

//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
