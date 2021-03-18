//==============================================================================
// Notes
//==============================================================================
// drivers::touch::cst8165.rs
// Register definitions for the CST8165 Touch Sensor 

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum REGISTER { 
	START 								= 0x0
}

//==============================================================================
// Variables
//==============================================================================
#[allow(dead_code)]
pub const EVENT_COUNT: usize 			= 10;

#[allow(dead_code)]
pub const I2C_ADDRESS: u8 				= 0x15;

#[allow(dead_code)]
pub const WHO_AM_I_VALUE: u8			= 0x44;