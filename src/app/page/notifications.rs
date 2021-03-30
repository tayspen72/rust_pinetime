//==============================================================================
// Notes
//==============================================================================
// app::page::notifications.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use crate::app::info;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================


//==============================================================================
// Variables
//==============================================================================


//==============================================================================
// Public Functions
//==============================================================================
#[allow(dead_code)]
pub fn event_handler(d: &mut info::DeviceInfo){
	if d.change_flags.touch_event {
		();
	}

	if d.change_flags.button_press {
		();
	}
}

#[allow(dead_code)]
pub fn print_page() {

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
#[allow(dead_code)]
pub fn task_handler() {

}