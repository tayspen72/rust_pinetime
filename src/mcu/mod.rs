//==============================================================================
// Notes
//==============================================================================
// mcu::mod.rs

//==============================================================================
// Crates and Mods
//==============================================================================
pub mod gpio;
pub mod i2c;
pub mod input;
pub mod rtc;
pub mod spi;
pub mod timer;

use nrf52832_pac;
use crate::drivers::app;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================


//==============================================================================
// Variables
//==============================================================================


//==============================================================================
// Public Functions
//==============================================================================
pub fn init(wake_interval: rtc::WakeInterval) {
	let peripherals = nrf52832_pac::Peripherals::take().unwrap();

	peripherals.CLOCK.tasks_hfclkstart.write(|w| unsafe { w.bits(1) });
	while peripherals.CLOCK.events_hfclkstarted.read().bits() == 0 {};

	gpio::init(peripherals.P0);

	i2c::init(peripherals.TWI0);
	rtc::init(peripherals.RTC0, &peripherals.CLOCK, wake_interval);
	spi::init(peripherals.SPIM0);
	timer::init(peripherals.TIMER0);

}

//==============================================================================
// Private Functions
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler(_d: &app::DeviceInfo) {
	rtc::task_handler();
	timer::task_handler();
}