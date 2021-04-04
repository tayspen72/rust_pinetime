//==============================================================================
// Notes
//==============================================================================
// drivers::debug.rs
// The debug library is meant to be a scrolling log of entries. The log will 
// need to be built. Later.
// The log can be hidden in real-time, as needed. Maybe with a swipe up action?

//==============================================================================
// Crates and Mods
//==============================================================================
use crate::app::{info, page};
use super::lcd::{lcd_api, font};

//==============================================================================
// Enums, Structs, and Types
//=============================================================================
#[derive(Copy, Clone)]
struct LogLine{
	active: bool,
	stale: bool,
	line: [u8; 24]
}

//==============================================================================
// Variables
//==============================================================================
 const DEBUG_INITIAL_X: u16 = 0;
 const DEBUG_INITIAL_Y: u16 = 0;
 const DEBUG_SCALE: u16 = 2;
 const DEBUG_BACKGROUND: lcd_api::Color = lcd_api::Color::Black;
 const DEBUG_FOREGROUND: lcd_api::Color = lcd_api::Color::White;
 const DEBUG_WELCOME: &'static str = "** Log Output Window **";

const LOG_PREFIX_LENGTH: usize = 3;
const LOG_MAX_LENGTH: usize = 24;
const LOG_ACTUAL_LEN: usize = LOG_MAX_LENGTH - LOG_PREFIX_LENGTH;
const LOG_LINE_ENTRIES: usize = 15;
static mut LOG_LINES_ACTIVE: usize = 0;
static mut LOG_LINES:[LogLine; LOG_LINE_ENTRIES] = [
	LogLine { active: false, stale: true, line: [ 0x00; LOG_MAX_LENGTH ] };
	LOG_LINE_ENTRIES
];

//==============================================================================
// Public Functions
//==============================================================================
pub fn init() {
	// Push the welcome message
	push_log(DEBUG_WELCOME);
}

pub fn make_stale() {
	unsafe {
		for i in 0..LOG_LINES_ACTIVE {
			LOG_LINES[i].stale = true;
		}
	}
}

#[allow(dead_code)]
pub fn push_log(string: &'static str) {
	unsafe { 
		if LOG_LINES_ACTIVE == LOG_LINE_ENTRIES {
			pop_log();
		}
			
		let index = LOG_LINES_ACTIVE;
		LOG_LINES_ACTIVE = LOG_LINES_ACTIVE + 1;
		
		let len = if string.len() < LOG_ACTUAL_LEN { string.len()} else { LOG_ACTUAL_LEN };
		let string = string.as_bytes();

		LOG_LINES[index].active = true;
		LOG_LINES[index].stale = true; 
		
		// Copy bytes from string into the log lines object
		for i in 0..len {
			LOG_LINES[index].line[i] = string[i];
		}
		if len < LOG_ACTUAL_LEN {
			LOG_LINES[index].line[len] = 0;
		}
	}
}

#[allow(dead_code)]
pub fn push_log_number(string: &'static str, num: &u32) {
	unsafe { 
		if LOG_LINES_ACTIVE == LOG_LINE_ENTRIES {
			pop_log();
		}
		
		let index = LOG_LINES_ACTIVE;
		LOG_LINES_ACTIVE = LOG_LINES_ACTIVE + 1;
		
		let string_len = if string.len() < LOG_ACTUAL_LEN { string.len()} else { LOG_ACTUAL_LEN };
		let string = string.as_bytes();
		let num_len = get_num_len(*num);

		LOG_LINES[index].active = true;
		LOG_LINES[index].stale = true; 

		// Copy bytes from string into the log lines object
		for i in 0..string_len {
			LOG_LINES[index].line[i] = string[i];
		}
		// Copy in number as ascii
		let mut div: u32 = 1;
		for i in string_len..(string_len+num_len) {
			if i == LOG_ACTUAL_LEN {
				break;
			}
			else {
				LOG_LINES[index].line[i] = (0x30 + ((num / div) % 10)) as u8;
				div *= 10;
			}
		}

		if string_len + num_len <= LOG_ACTUAL_LEN {
			LOG_LINES[index].line[string_len+num_len+3] = 0;
		}
	}
}

//==============================================================================
// Private Functions
//==============================================================================
fn clear_line(line_number: usize) {
	let y = DEBUG_INITIAL_Y + ((line_number as u16) * font::MINIMAL_CHARACTER_HEIGHT * DEBUG_SCALE);
	lcd_api::fill_rectangle(0, 240, y, font::MINIMAL_CHARACTER_HEIGHT * DEBUG_SCALE, DEBUG_BACKGROUND);
}

fn get_line_length(line_number: usize) -> usize {
	let line = unsafe { LOG_LINES[line_number].line }; 
	for i in 0..line.len() {
		if line[i] == 0 {
			return i + 1;
		}
	}

	line.len()
}

fn get_num_len(mut num: u32) -> usize {
	let mut len: usize = 1;
	num /= 10;
	while num > 0 {
		len += 1;
		num /= 10;
	}

	len
}

fn pop_log() {
	unsafe {
		// Show that a line has just been popped
		LOG_LINES_ACTIVE = LOG_LINES_ACTIVE - 1;

		// Shift all entries up one - leaving the bottom entry available
		// Start at 1 to always show the header on row 0
		for i in 1..(LOG_LINES_ACTIVE) {
			LOG_LINES[i].active = true;
			LOG_LINES[i].stale = true;
			LOG_LINES[i].line = LOG_LINES[i+1].line;
		}
		LOG_LINES[LOG_LINES_ACTIVE].active = false;

	}
}

fn write_line(line_number: usize) {
	let y = DEBUG_INITIAL_Y + ((line_number as u16) * font::MINIMAL_CHARACTER_HEIGHT * DEBUG_SCALE);
	let len = get_line_length(line_number);

	unsafe { 
		font::write_minimal_line(&LOG_LINES[line_number].line[0..len], DEBUG_INITIAL_X, y, DEBUG_FOREGROUND, DEBUG_BACKGROUND, DEBUG_SCALE);

		// Update the stale line flag showing it has been displayed
		LOG_LINES[line_number].stale = false;
	}
}

//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler(d: &info::DeviceInfo) {
	if let page::AppPage::Log = d.app_page {
		unsafe {
			for i in 0..LOG_LINES_ACTIVE {
				// If log lines are current, do nothing
				if !LOG_LINES[i].active {
					return;
				}

				if LOG_LINES[i].stale {
					clear_line(i);
					write_line(i);
				}
			}
		}
	}
}
