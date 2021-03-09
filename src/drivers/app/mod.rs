//==============================================================================
// Notes
//==============================================================================
// drivers::app::mod.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use cortex_m::asm::wfi;

pub mod app;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
pub struct DeviceInfoFlags{
	pub debug_log_active: bool,
}

pub struct DeviceInfo {
	pub flags: DeviceInfoFlags
}

pub enum AppState {
    BusyLcd,
    BusyTimer,
    Idle,
}

//==============================================================================
// Macros
//==============================================================================


//==============================================================================
// Variables
//==============================================================================
static mut DEVICE_INFO: bool = false;

//==============================================================================
// Implementations
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
        DeviceInfo {
			flags: DeviceInfoFlags { 
				debug_log_active: false 
			}
		}
	}
}

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