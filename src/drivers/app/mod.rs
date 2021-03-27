//==============================================================================
// Notes
//==============================================================================
// drivers::app::mod.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use cortex_m::asm::wfi;
use crate::drivers;
pub mod app;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
pub struct DeviceInfoChangeFlags{
	pub battery_voltage: bool,
	pub button_press: bool,
	pub charger_state: bool,
	pub debug_log: bool,
	pub time_change: bool,
	pub touch_event: bool,
}

pub struct DeviceInfoFlags{
	pub charger_connected: bool,
	pub button_pressed: bool,
	pub debug_log_active: bool,
}

pub struct DeviceInfo {
	pub change_flags: DeviceInfoChangeFlags,
	pub flags: DeviceInfoFlags,

	pub battery_voltage: u16,
	pub time: drivers::clock::Time,
	pub touch: drivers::touch::TouchEvent
}

pub enum AppState {
	BusyLcd,
	BusyTimer,
	Idle,
}

//==============================================================================
// Variables
//==============================================================================
static mut DEVICE_INFO: bool = false;

const DEVICE_INFO_DEFAULTS: DeviceInfo = DeviceInfo {
	change_flags: DeviceInfoChangeFlags {
		battery_voltage: false,
		button_press: false,
		charger_state: false,
		debug_log: false,
		time_change: false,
		touch_event: false
	},
	flags: DeviceInfoFlags {
		charger_connected: false,
		button_pressed: false,
		debug_log_active: true,
	},
	battery_voltage: 0,
	time: drivers::clock::Time {
		hours: 0,
		minutes: 0, 
		seconds: 0
	},
	touch: drivers::touch::TouchEvent {
		gesture: drivers::touch::Gesture::Unknown,
		event: drivers::touch::Event::Unknown,
		x: 0,
		y: 0,
		pressure: 0
	}
};

//==============================================================================
// Public Functions
//==============================================================================
impl DeviceInfo {
	pub fn take() -> Option<Self> {
		cortex_m::interrupt::free(|_| {
			if unsafe { DEVICE_INFO } {
				None
			} else {
				Some(unsafe { DeviceInfo::steal() })
			}
		})
	}
	pub unsafe fn steal() -> Self {
		DEVICE_INFO = true;
		DEVICE_INFO_DEFAULTS
	}
}

//==============================================================================
// Private Functions
//==============================================================================


//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler() {
	let state = app::get_state();
	match state {
		AppState::Idle => wfi(),
		_ => ()
	}
}