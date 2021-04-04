//==============================================================================
// Notes
//==============================================================================
// drivers::battery.rs
// Wrapper around the gpio pins for handling button presses

//==============================================================================
// Crates and Mods
//==============================================================================
use crate::app::info;
use crate::config;
use super::log;
use crate::mcu::{adc, gpio, input, rtc};

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum BatteryLevel{
	Level4 = 4,
	Level3 = 3,
	Level2 = 2,
	Level1 = 1,
	Level0 = 0
}

//==============================================================================
// Variables
//==============================================================================
const BATTERY_CHECK_INTERVAL: u32 = 30; //60 * 5;	// 5 Minutes

const CHARGER_CONNECT_PIN: input::PinConfig = input::PinConfig {
	pin: config::CHARGER_CONNECTED_PIN,
	polarity: nrf52832_pac::gpiote::config::POLARITY_A::HITOLO,
	pull: nrf52832_pac::p0::pin_cnf::PULL_A::PULLUP,
	callback: &connect_handler,
	real_time_callback: false
};

static mut CHARGER_CONNECTED: bool = false;

//==============================================================================
// Public Functions
//==============================================================================
pub fn init() {
	input::init_pin(CHARGER_CONNECT_PIN);
	connect_handler();
}

//==============================================================================
// Private Functions
//==============================================================================
#[allow(dead_code)]
fn connect_handler() {
	unsafe { 
		CHARGER_CONNECTED = match gpio::get_pin_state(config::CHARGER_CONNECTED_PIN) {
			gpio::PinState::PinHigh => false,
			gpio::PinState::PinLow => true,
		}};
}

fn get_battery_level(voltage: u16) -> BatteryLevel {
	if voltage > 350 {
		BatteryLevel::Level4
	}
	else if voltage > 340 {
		BatteryLevel::Level3
	}
	else if voltage > 330 {
		BatteryLevel::Level2
	}
	else if voltage > 320 {
		BatteryLevel::Level1
	}
	else {
		BatteryLevel::Level0
	}
}

fn get_battery_voltage() -> u16 {
	// Voltage divider by half (R1 = R2 = 1M)
	// 12-bit number
	// Gain of 1/6
	// Ref voltage of internal 0.6
	// Voltage range: 0.6 * (1/6) = 3.6V

	let raw = adc::read_adc() as u32;

	// raw * 2000 / (4095.0 / 3.6)
	let voltage: u32 = (raw * 2000) / 1137;
	voltage as u16
}

//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler(d: &mut info::DeviceInfo) {
	static mut LAST_BATTERY_TIMESTAMP: u32 = 0;
	static mut LAST_BATTERY_STATE: u8 = 0;
	static mut LAST_BATTERY_VOLTAGE: u16 = 0;
	static mut LAST_CHARGER_CONNECTED: bool = false;

	if d.change_flags.battery_voltage {
		d.change_flags.battery_voltage = false;
	}
	if d.change_flags.charger_state {
		d.change_flags.charger_state = false;
	}

	unsafe {
		if rtc::get_timediff(LAST_BATTERY_TIMESTAMP) > BATTERY_CHECK_INTERVAL {
			LAST_BATTERY_TIMESTAMP = rtc::get_timestamp();
			let tmp_voltage = get_battery_voltage();
			
			log::push_log_number("Battery: ", &(tmp_voltage as u32));
			
			if LAST_BATTERY_VOLTAGE != tmp_voltage {
				LAST_BATTERY_VOLTAGE = tmp_voltage;
				d.change_flags.battery_voltage = true;
				d.battery_voltage = tmp_voltage;

				let tmp_level: u8 = get_battery_level(tmp_voltage) as u8;
				if LAST_BATTERY_STATE != tmp_level {
					LAST_BATTERY_STATE = tmp_level;
					d.battery_level = get_battery_level(tmp_voltage);
				}
			}
		}

		if LAST_CHARGER_CONNECTED != CHARGER_CONNECTED {
			LAST_CHARGER_CONNECTED = CHARGER_CONNECTED;
			d.change_flags.charger_state = true;
			d.flags.charger_connected = CHARGER_CONNECTED;

			log::push_log_number("Charger state: ", &( 
				if CHARGER_CONNECTED { 1 } else { 0 }));
		}
	}
}