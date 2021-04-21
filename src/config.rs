//==============================================================================
// Notes
//==============================================================================
// config.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use nrf52832_pac::{twi0, spi0, spim0, uart0};

//==============================================================================
// Accelerometer
//==============================================================================
#[allow(dead_code)] pub const ACCEL_I2C_SDA_PIN: u8 	= I2C_SDA_PIN;
#[allow(dead_code)] pub const ACCEL_I2C_SCL_PIN: u8 	= I2C_SCL_PIN;
#[allow(dead_code)] pub const ACCEL_I2C_FREQUENCY: twi0::frequency::FREQUENCY_A	= I2C_FREQUENCY;
#[allow(dead_code)] pub const ACCEL_INT_PIN: u8 		= 8;

//==============================================================================
// Battery 
//==============================================================================
pub const ADC_RAM_BUFFER: u32	= 0x2000_FBF0;
pub const ADC_RAM_BUFFER_LEN: usize	= 8;
pub const BATTERY_ADC_CHANNEL: u8 	= 7;
pub const BATTERY_ADC_PIN: u8 		= 31;
pub const CHARGER_CONNECTED_PIN: u8 = 12;

//==============================================================================
// DEBUG
//==============================================================================


//==============================================================================
// Flash
//==============================================================================
// pub const FLASH_CS_PIN: u8 = 3;

//==============================================================================
// I2C
//==============================================================================
pub const I2C_SCL_PIN: u8 			= 7;
pub const I2C_SDA_PIN: u8 			= 6;
pub const I2C_FREQUENCY: twi0::frequency::FREQUENCY_A = twi0::frequency::FREQUENCY_A::K100;

//==============================================================================
// LCD
//==============================================================================
pub const LCD_BACKLIGHT_LOW: u8 	= 14;
pub const LCD_BACKLIGHT_MID: u8 	= 22;
pub const LCD_BACKLIGHT_HIGH: u8 	= 23;
pub const LCD_CS_PIN: u8 			= 25;
pub const LCD_DCX_PIN: u8 			= 18;
pub const LCD_RESET_PIN: u8 		= 26;

//==============================================================================
// Push Button
//==============================================================================
pub const PUSH_BUTTON_IN_PIN: u8 	= 13;
pub const PUSH_BUTTON_OUT_PIN: u8 	= 15;

//==============================================================================
// RTC
//==============================================================================


//==============================================================================
// SPI
//==============================================================================
pub const SPI_CPHA: spi0::config::CPHA_A = spi0::config::CPHA_A::TRAILING;
pub const SPI_CPOL: spi0::config::CPOL_A = spi0::config::CPOL_A::ACTIVELOW;
pub const SPI_FREQUENCY: spi0::frequency::FREQUENCY_A = spi0::frequency::FREQUENCY_A::M8;
pub const SPI_ORDER: spi0::config::ORDER_A = spi0::config::ORDER_A::MSBFIRST;
pub const SPI_MOSI_PIN: u8 			= 3;
pub const SPI_MISO_PIN: u8 			= 4;
pub const SPI_SCLK_PIN: u8 			= 2;

//==============================================================================
// SPI and SPIM
//==============================================================================
pub const SPIM_CPHA: spim0::config::CPHA_A = spim0::config::CPHA_A::TRAILING;
pub const SPIM_CPOL: spim0::config::CPOL_A = spim0::config::CPOL_A::ACTIVELOW;
pub const SPIM_FREQUENCY: spim0::frequency::FREQUENCY_A = spim0::frequency::FREQUENCY_A::M8;
pub const SPIM_ORDER: spim0::config::ORDER_A = spim0::config::ORDER_A::MSBFIRST;
pub const SPIM_DMA_MAX: u32 = 0x20010000;
pub const SPIM_DMA_MIN: u32 = 0x20000000;
pub const SPIM_DMA_RX_PTR: u32 = 0x2000FE00;
pub const SPIM_DMA_TX_PTR_A: u32 = 0x2000FD00;
pub const SPIM_DMA_TX_PTR_B: u32 = 0x2000FC00;
pub const SPIM_DMA_SIZE: u32 = 256;

//==============================================================================
// Touch Sensor
//==============================================================================
pub const TOUCH_I2C_ADDRESS: u8 	= 0x15;
pub const TOUCH_INT_PIN: u8 		= 28;
pub const TOUCH_RESET_PIN: u8 		= 10;

//==============================================================================
// Uart
//==============================================================================
#[allow(dead_code)] pub const UART_CTS_PIN: Option<u8> = Some(7);
#[allow(dead_code)] pub const UART_RTS_PIN: Option<u8> = Some(5);
#[allow(dead_code)] pub const UART_RX_PIN: u8 = 8;
#[allow(dead_code)] pub const UART_TX_PIN: u8 = 6;
#[allow(dead_code)] pub const UART_BAUD: uart0::baudrate::BAUDRATE_A = uart0::baudrate::BAUDRATE_A::BAUD115200;
#[allow(dead_code)] pub const UART_PARITY: uart0::config::PARITY_A = uart0::config::PARITY_A::EXCLUDED;
#[allow(dead_code)] pub const UART_ECHO: bool = true;
#[allow(dead_code)] pub const UART_RX_BUFFER_LENGTH: usize = 32;
