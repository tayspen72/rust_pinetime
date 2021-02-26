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
pub const PUSH_BUTTON_PIN: u8 = 0; //TODO: Get the right pin

pub const I2C_SCL_PIN: u8 = 0; //TODO: Get the right pin
pub const I2C_SDA_PIN: u8 = 0; //TODO: Get the right pin
pub const I2C_FREQUENCY: twi0::frequency::FREQUENCY_A = twi0::frequency::FREQUENCY_A::K400;
pub const I2C_ADDRESS: u8 = 0; //TODO: Get the right pin

pub const SPI_SCLK_PIN: u8 = 11;
pub const SPI_SEL_PIN: u8 = 12;
pub const SPI_MOSI_PIN: u8 = 13;
pub const SPI_MISO_PIN: u8 = 14;
pub const SPI_FREQUENCY: spi0::frequency::FREQUENCY_A = spi0::frequency::FREQUENCY_A::M8;
pub const SPI_ORDER: spi0::config::ORDER_A = spi0::config::ORDER_A::MSBFIRST;
pub const SPI_CPHA: spi0::config::CPHA_A = spi0::config::CPHA_A::TRAILING;
pub const SPI_CPOL: spi0::config::CPOL_A = spi0::config::CPOL_A::ACTIVELOW;

pub const UART_CTS_PIN: Option<u8> = Some(7);
pub const UART_RTS_PIN: Option<u8> = Some(5);
pub const UART_RX_PIN: u8 = 8;
pub const UART_TX_PIN: u8 = 6;
pub const UART_BAUD: uart0::baudrate::BAUDRATE_A = uart0::baudrate::BAUDRATE_A::BAUD115200;
pub const UART_PARITY: uart0::config::PARITY_A = uart0::config::PARITY_A::EXCLUDED;
pub const UART_ECHO: bool = true;
pub const UART_RX_BUFFER_LENGTH: usize = 32;

//==============================================================================
// Implementations
//==============================================================================


//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
