//==============================================================================
// Notes
//==============================================================================
// app::app.rs
// Top level app behavior

//==============================================================================
// Crates and Mods
//==============================================================================
use super::{display, info, page};
use crate::drivers;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================


//==============================================================================
// Variables
//==============================================================================


//==============================================================================
// Public Functions
//==============================================================================


//==============================================================================
// Private Functions
//==============================================================================


//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler(d: &mut info::DeviceInfo){
	// Call the needed sub task handlers
	display::task_handler(d);

	// Handle all input events and route them to pages as needed
	match d.app_page {
		page::AppPage::Home => {
			if d.change_flags.time_change {
				drivers::clock::write_time();
			}

			if d.change_flags.touch_event {
				match d.touch.gesture {
					drivers::touch::Gesture::SlideUp => {
						d.app_page = page::AppPage::Log;
						d.change_flags.app_page = true;
					},
					drivers::touch::Gesture::SlideDown => (),
					drivers::touch::Gesture::SlideRight => (),
					drivers::touch::Gesture::SlideLeft => (),
					// TODO: For now, don't do anything else
					_ => (),
				}
			}
		},
		page::AppPage::Notifications => (),
		page::AppPage::Log => {
			if d.change_flags.touch_event {
				match d.touch.gesture {
					drivers::touch::Gesture::SlideDown => {
						d.app_page = page::AppPage::Log;
						d.change_flags.app_page = true;
					},
					
					// TODO: For now, don't do anything else
					_ => (),
				}
			}
		}
		page::AppPage::Settings => (),
		page::AppPage::Startup => {
			// If in startup, change to home
			d.app_page = page::AppPage::Home;
			d.change_flags.app_page = true;
		},
	}
}