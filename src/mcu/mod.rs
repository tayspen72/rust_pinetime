//==============================================================================
// Notes
//==============================================================================
// mcu::mod.rs

//==============================================================================
// Crates and Mods
//==============================================================================
pub mod adc;
pub mod gpio;
pub mod i2c;
pub mod input;
pub mod rtc;
pub mod spi;
pub mod timer;

use cortex_m;
use nrf52832_pac;
use crate::app::info;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[allow(dead_code)]
pub enum McuState {
	AdcBusy,
	SpiBusy,
	TimerBusy,

	Idle
}

//==============================================================================
// Variables
//==============================================================================


//==============================================================================
// Public Functions
//==============================================================================
pub fn init(wake_interval: rtc::WakeInterval) {
	let peripherals = nrf52832_pac::Peripherals::take().unwrap();
	// let cortex = cortex_m::Peripherals::take().unwrap();

	peripherals.CLOCK.tasks_hfclkstart.write(|w| unsafe { w.bits(1) });
	while peripherals.CLOCK.events_hfclkstarted.read().bits() == 0 {};

	adc::init(peripherals.SAADC);
	gpio::init(peripherals.P0);
	input::init(peripherals.GPIOTE);
	i2c::init(peripherals.TWI1);
	rtc::init(peripherals.RTC0, &peripherals.CLOCK, wake_interval);
	spi::init(peripherals.SPI0, peripherals.SPIM0);
	timer::init(peripherals.TIMER0);
}

pub fn get_busy() -> McuState {
	if adc::get_busy() {
		return McuState::AdcBusy;
	}
	
	if spi::get_busy() {
		return McuState::SpiBusy;
	}
	if timer::get_busy() {
		return McuState::TimerBusy;
	}
	McuState::Idle
}

pub fn restart() {
	cortex_m::peripheral::SCB::sys_reset();
}

//==============================================================================
// Private Functions
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler(_d: &info::DeviceInfo) {
	input::task_handler();
}