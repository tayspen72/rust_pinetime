//==============================================================================
// Notes
//==============================================================================
// app::page::home.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use crate::app::{info, page};
use crate::drivers::clock;
use crate::drivers::lcd::{lcd_api, font};
use crate::drivers::touch::Gesture;
use crate::mcu;
use crate::mcu::rtc;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================


//==============================================================================
// Variables
//==============================================================================
const BUTTON_HELD_RESTART_WARN: u32 = 1;
const BUTTON_HELD_RESTART: u32 = 5;

//==============================================================================
// Public Functions
//==============================================================================
pub fn start_page() {
	clock::write_time();
}

//==============================================================================
// Private Functions
//==============================================================================
fn button_held_handler(last_press_time: u32){
	let time_diff = rtc::get_timediff(last_press_time);

	if time_diff >= BUTTON_HELD_RESTART {
		mcu::restart();
	}
	else if time_diff >= BUTTON_HELD_RESTART_WARN {
		// Print the restart warning
		font::write_minimal_line(
			"Hold button".as_bytes(),
			20, 
			95, 
			lcd_api::Color::White,
			lcd_api::Color::Black, 
			3
		);
		font::write_minimal_line(
			"to restart!".as_bytes(),
			20, 
			119, 
			lcd_api::Color::White,
			lcd_api::Color::Black, 
			3
		);
	}

}

//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler(d: &mut info::DeviceInfo) {
	static mut PRESS_TIMER_RUNNING: bool = false;
	static mut LAST_PRESS_TIME: u32 = 0;

	// First, check for button hold timout
	unsafe {
		if PRESS_TIMER_RUNNING {
			button_held_handler(LAST_PRESS_TIME);
		}
	}

	if d.change_flags.button_press {
		();
	}
	
	if d.change_flags.time_change {
		clock::update_time();
	}

	if d.change_flags.touch_event {
		match d.touch.gesture {
			Gesture::SlideDown => {
				d.app_page = page::AppPage::Settings;
				page::change_page(d);
			},
			Gesture::SlideUp => {
				d.app_page = page::AppPage::Log;
				page::change_page(d);
			},
			_ => (),
		}
	}

	if d.change_flags.button_press {
		if d.flags.button_pressed {
			unsafe { 
				PRESS_TIMER_RUNNING = true;
				LAST_PRESS_TIME = rtc::get_timestamp();
			}
		}
		else {
			//TOOD: Need to cler restart prompt if button is released
			();
		}
	}
}