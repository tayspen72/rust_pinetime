//==============================================================================
// Notes
//==============================================================================
// drivers::lcd::lcd.rs
// LCD Essential Functions

//==============================================================================
// Crates and Mods
//==============================================================================
use crate::config;
use crate::mcu::{gpio, spi, timer};
use super::st7789;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[allow(dead_code)]
pub enum BacklightBrightness {
	Brightness0,
	Brightness1,
	Brightness2,
	Brightness3,
	Brightness4,
	Brightness5,
	Brightness6,
	Brightness7
}

//==============================================================================
// Macros
//==============================================================================


//==============================================================================
// Variables
//==============================================================================
static mut _INTIIALIZED: bool = false;

//==============================================================================
// Implementations
//==============================================================================
pub fn init(p: &nrf52832_pac::Peripherals) {
	if unsafe { _INTIIALIZED } {
		return;
	}

	spi::init(p, get_spiline());

	configure(p, get_spiline());

	gpio::pin_setup(p, config::LCD_BACKLIGHT_LOW, nrf52832_pac::p0::pin_cnf::DIR_A::OUTPUT, nrf52832_pac::p0::pin_cnf::PULL_A::PULLUP, gpio::PinState::PinLow);
	gpio::pin_setup(p, config::LCD_BACKLIGHT_MID, nrf52832_pac::p0::pin_cnf::DIR_A::OUTPUT, nrf52832_pac::p0::pin_cnf::PULL_A::PULLUP, gpio::PinState::PinLow);
	gpio::pin_setup(p, config::LCD_BACKLIGHT_HIGH, nrf52832_pac::p0::pin_cnf::DIR_A::OUTPUT, nrf52832_pac::p0::pin_cnf::PULL_A::PULLUP, gpio::PinState::PinLow);
	set_backlight(p, BacklightBrightness::Brightness7);

	unsafe { _INTIIALIZED = true; }
}

fn configure(p: &nrf52832_pac::Peripherals, spiline: &spi::SpiLine) {
	// Enter safe reset sequence
	gpio::set_pin_state(p, config::LCD_RESET_PIN, gpio::PinState::PinHigh);
	timer::delay(p, 5);
	gpio::set_pin_state(p, config::LCD_RESET_PIN, gpio::PinState::PinLow);
	timer::delay(p, 20);
	gpio::set_pin_state(p, config::LCD_RESET_PIN, gpio::PinState::PinHigh);
	timer::delay(p, 150);

	// Also initiate a software reset - just to be safe
	write_command(p, spiline, st7789::COMMAND_A::SW_RESET);
	timer::delay(p, 150);

	// Exit sleep
	write_command(p, spiline, st7789::COMMAND_A::SLEEP_OUT);
	timer::delay(p, 150);

	write_command(p, spiline, st7789::COMMAND_A::NORMAL_MODE);
	
	// Write memory data format: 
	//  RGB, left to right, top to bottom, logical direction of memory pointer updates
	write_command(p, spiline, st7789::COMMAND_A::MEMORY_DATA_ACCESS_CONTROL);
	write_data(p, spiline, &[ 0x08 ]);

	// Define pixel interfacing format:
	//  5-6-5 for 65k color options
	write_command(p, spiline, st7789::COMMAND_A::INTERFACE_PIXEL_FORMAT);
	write_data(p, spiline, &[ 0x55 ]);
	timer::delay(p, 10);

	write_command(p, spiline, st7789::COMMAND_A::PORCH_SETTING);
	write_data(p, spiline, &[ 0x0c, 0x0c, 0x00, 0x33, 0x33 ]);

	write_command(p, spiline, st7789::COMMAND_A::GATE_CONTROL);
	write_data(p, spiline, &[ 0x35 ]);

	write_command(p, spiline, st7789::COMMAND_A::GATE_ON_TIMING_ADJUSTMENT);
	write_data(p, spiline, &[ 0x28 ]);

	write_command(p, spiline, st7789::COMMAND_A::LCM_CONTROL);
	write_data(p, spiline, &[ 0x0C ]);

	write_command(p, spiline, st7789::COMMAND_A::VDV_VRH_CMD_ENABLE);
	write_data(p, spiline, &[ 0x01, 0xFF ]);

	write_command(p, spiline, st7789::COMMAND_A::VRH_SET);
	write_data(p, spiline, &[ 0x01 ]);

	write_command(p, spiline, st7789::COMMAND_A::VDV_SET);
	write_data(p, spiline, &[ 0x20 ]);

	write_command(p, spiline, st7789::COMMAND_A::FRAME_RATE_CONTROL_2);
	write_data(p, spiline, &[ 0x0F ]);

	write_command(p, spiline, st7789::COMMAND_A::POWER_CONTROL_1);
	write_data(p, spiline, &[ 0xA4, 0xA1 ]);

	write_command(p, spiline, st7789::COMMAND_A::POSITIVE_VOLTAGE_GAMMA_CONTROL);
	write_data(p, spiline, &[ 0xd0, 0x00, 0x02, 0x07, 0x0a, 0x28, 0x32, 0x44, 0x42, 0x06, 0x0e, 0x12, 0x14, 0x17 ]);
	
	write_command(p, spiline, st7789::COMMAND_A::NEGATIVE_VOLTAGE_GAMMA_CONTROL);
	write_data(p, spiline, &[ 0xd0, 0x00, 0x02, 0x07, 0x0a, 0x28, 0x31, 0x54, 0x47, 0x0e, 0x1c, 0x17, 0x1b, 0x1e ]); 	
	
	write_command(p, spiline, st7789::COMMAND_A::DISPLAY_INVERSION_ON);
	
	write_command(p, spiline, st7789::COMMAND_A::DISPLAY_BRIGHTNESS);
	write_data(p, spiline, &[ 0x3F ]);	//initial 25% brightness

	write_command(p, spiline, st7789::COMMAND_A::GAMMA);
	write_data(p, spiline, &[ 0x04 ]);

	// Explicitly end this bulk write
	gpio::set_pin_state(p, config::LCD_CS_PIN, gpio::PinState::PinHigh);

	timer::delay(p, 120);
	
	write_command(p, spiline, st7789::COMMAND_A::DISPLAY_ON);
	
	timer::delay(p, 120);
}

fn get_spiline() -> &'static spi::SpiLine {
	static SPI_LINE: spi::SpiLine = spi::SpiLine {
		sclk_pin: config::SPI_SCLK_PIN,
		sel_pin: config::SPI_SEL_PIN,
		mosi_pin: config::SPI_MOSI_PIN,
		miso_pin: config::SPI_MISO_PIN,
		frequency: config::SPI_FREQUENCY,
		order: config::SPI_ORDER,
		cpha: config::SPI_CPHA,
		cpol: config::SPI_CPOL,
	};

	&SPI_LINE
}

pub fn set_backlight(p: &nrf52832_pac::Peripherals, backlight: BacklightBrightness) {
	let states = match backlight {
		BacklightBrightness::Brightness0 => [ gpio::PinState::PinHigh, gpio::PinState::PinHigh, gpio::PinState::PinHigh ],
		BacklightBrightness::Brightness1 => [ gpio::PinState::PinHigh, gpio::PinState::PinHigh, gpio::PinState::PinLow ],
		BacklightBrightness::Brightness2 => [ gpio::PinState::PinHigh, gpio::PinState::PinLow, gpio::PinState::PinHigh ],
		BacklightBrightness::Brightness3 => [ gpio::PinState::PinHigh, gpio::PinState::PinLow, gpio::PinState::PinLow ],
		BacklightBrightness::Brightness4 => [ gpio::PinState::PinLow, gpio::PinState::PinHigh, gpio::PinState::PinHigh ],
		BacklightBrightness::Brightness5 => [ gpio::PinState::PinLow, gpio::PinState::PinHigh, gpio::PinState::PinLow ],
		BacklightBrightness::Brightness6 => [ gpio::PinState::PinLow, gpio::PinState::PinLow, gpio::PinState::PinHigh ],
		BacklightBrightness::Brightness7 => [ gpio::PinState::PinLow, gpio::PinState::PinLow, gpio::PinState::PinLow ]
	};

	gpio::set_pin_state(p, config::LCD_BACKLIGHT_LOW, states[0]);
	gpio::set_pin_state(p, config::LCD_BACKLIGHT_LOW, states[1]);
	gpio::set_pin_state(p, config::LCD_BACKLIGHT_LOW, states[2]);
}

fn write_command(p: &nrf52832_pac::Peripherals, spiline: &spi::SpiLine, command: st7789::COMMAND_A) {
	gpio::set_pin_state(p, config::LCD_CS_PIN, gpio::PinState::PinLow);
	gpio::set_pin_state(p, config::LCD_DCX_PIN, gpio::PinState::PinLow);

	spi::tx_byte(p, spiline, command as u8);

	gpio::set_pin_state(p, config::LCD_DCX_PIN, gpio::PinState::PinHigh);
	gpio::set_pin_state(p, config::LCD_CS_PIN, gpio::PinState::PinHigh);
}

fn write_data(p: &nrf52832_pac::Peripherals, spiline: &spi::SpiLine, data: &[u8]) {
	gpio::set_pin_state(p, config::LCD_CS_PIN, gpio::PinState::PinLow);
	gpio::set_pin_state(p, config::LCD_DCX_PIN, gpio::PinState::PinHigh);

	spi::tx_data(p, spiline, data);

	gpio::set_pin_state(p, config::LCD_DCX_PIN, gpio::PinState::PinHigh);
}

//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
