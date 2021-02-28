//==============================================================================
// Notes
//==============================================================================
// config.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use nrf52832_pac::{twi0, spi0, uart0};

//==============================================================================
// Enums, Structs, and Types
//==============================================================================


//==============================================================================
// Macros
//==============================================================================


//==============================================================================
// Variables
//==============================================================================
#[allow(dead_code)] pub const ACCEL_I2C_SDA_PIN: u8 	= I2C_SDA_PIN;
#[allow(dead_code)] pub const ACCEL_I2C_SCL_PIN: u8 	= I2C_SCL_PIN;
#[allow(dead_code)] pub const ACCEL_I2C_FREQUENCY: twi0::frequency::FREQUENCY_A	= I2C_FREQUENCY;
#[allow(dead_code)] pub const ACCEL_INT_PIN: u8 		= 8;

pub const PUSH_BUTTON_IN_PIN: u8 	= 13;
pub const PUSH_BUTTON_OUT_PIN: u8 	= 15;

// pub const FLASH_CS_PIN: u8 = 3;

pub const I2C_SCL_PIN: u8 			= 6;
pub const I2C_SDA_PIN: u8 			= 7;
pub const I2C_FREQUENCY: twi0::frequency::FREQUENCY_A = twi0::frequency::FREQUENCY_A::K100;

pub const LCD_BACKLIGHT_LOW: u8 	= 14;
pub const LCD_BACKLIGHT_MID: u8 	= 22;
pub const LCD_BACKLIGHT_HIGH: u8 	= 23;
pub const LCD_CS_PIN: u8 			= 25;
pub const LCD_DCX_PIN: u8 			= 18;
pub const LCD_RESET_PIN: u8 		= 26;

pub const SPI_CPHA: spi0::config::CPHA_A = spi0::config::CPHA_A::TRAILING;
pub const SPI_CPOL: spi0::config::CPOL_A = spi0::config::CPOL_A::ACTIVELOW;
pub const SPI_FREQUENCY: spi0::frequency::FREQUENCY_A = spi0::frequency::FREQUENCY_A::M8;
pub const SPI_ORDER: spi0::config::ORDER_A = spi0::config::ORDER_A::MSBFIRST;
pub const SPI_MOSI_PIN: u8 			= 3;
pub const SPI_MISO_PIN: u8 			= 4;
pub const SPI_SCLK_PIN: u8 			= 2;

#[allow(dead_code)] pub const TOUCH_I2C_SDA_PIN: u8 	= I2C_SDA_PIN;
#[allow(dead_code)] pub const TOUCH_I2C_SCL_PIN: u8 	= I2C_SCL_PIN;
#[allow(dead_code)] pub const TOUCH_I2C_FREQUENCY: twi0::frequency::FREQUENCY_A	= I2C_FREQUENCY;
#[allow(dead_code)] pub const TOUCH_I2C_ADDRESS: u8 	= 0x00;
#[allow(dead_code)] pub const TOUCH_INT_PIN: u8 		= 28;
#[allow(dead_code)] pub const TOUCH_RESET_PIN: u8 		= 28;

#[allow(dead_code)] pub const UART_CTS_PIN: Option<u8> = Some(7);
#[allow(dead_code)] pub const UART_RTS_PIN: Option<u8> = Some(5);
#[allow(dead_code)] pub const UART_RX_PIN: u8 = 8;
#[allow(dead_code)] pub const UART_TX_PIN: u8 = 6;
#[allow(dead_code)] pub const UART_BAUD: uart0::baudrate::BAUDRATE_A = uart0::baudrate::BAUDRATE_A::BAUD115200;
#[allow(dead_code)] pub const UART_PARITY: uart0::config::PARITY_A = uart0::config::PARITY_A::EXCLUDED;
#[allow(dead_code)] pub const UART_ECHO: bool = true;
#[allow(dead_code)] pub const UART_RX_BUFFER_LENGTH: usize = 32;

//==============================================================================
// Implementations
//==============================================================================


//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
