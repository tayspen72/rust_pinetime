//==============================================================================
// Notes
//==============================================================================
// app::app.rs
// Top level app behavior

//==============================================================================
// Crates and Mods
//==============================================================================
use super::info;
use crate::drivers;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[allow(dead_code)]
pub enum AppPage {
	Home,
	Notifications,
	Log,
	Settings,
	Startup
}

//==============================================================================
// Variables
//==============================================================================


//==============================================================================
// Public Functions
//==============================================================================
pub fn page_handler(d: &mut info::DeviceInfo){
	// Handle all input events and route them to pages as needed
	match d.app_page {
		AppPage::Home => {
			if d.change_flags.time_change {
				drivers::clock::write_time();
			}

			if d.change_flags.touch_event {
				match d.touch.gesture {
					drivers::touch::Gesture::SlideUp => {
						d.app_page = AppPage::Log;
					},
					drivers::touch::Gesture::SlideDown => (),
					drivers::touch::Gesture::SlideRight => (),
					drivers::touch::Gesture::SlideLeft => (),
					// TODO: For now, don't do anything else
					_ => (),
				}
			}
		},
		AppPage::Notifications => (),
		AppPage::Log => {
			if d.change_flags.touch_event {
				match d.touch.gesture {
					drivers::touch::Gesture::SlideDown => {
						d.app_page = AppPage::Log
					},
					
					// TODO: For now, don't do anything else
					_ => (),
				}
			}
		}
		AppPage::Settings => (),
		AppPage::Startup => {
			// If in startup, change to home
			d.app_page = AppPage::Home;
			d.change_flags.app_page = true;
		},
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
