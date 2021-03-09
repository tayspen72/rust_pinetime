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
use super::app;
use super::lcd::{lcd_api, font};
use crate::mcu::rtc;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
struct LogLine{
	active: bool,
	timestamp: u32,
	line: &'static str
}

//==============================================================================
// Macros
//==============================================================================


//==============================================================================
// Variables
//==============================================================================
 const DEBUG_INITIAL_X: u16 = 0;
 const DEBUG_INITIAL_Y: u16 = 137;
 const DEBUG_SCALE: u16 = 2;
 const DEBUG_BACKGROUND: u16 = lcd_api::Color::Black as u16;
 const DEBUG_FOREGROUND: u16 = lcd_api::Color::White as u16;

static mut _LOG_LINES: [LogLine; 6] = [
	LogLine { active: true, timestamp: 0, line:  "**  Debug Output  **" },
	LogLine { active: false, timestamp: 0, line: "--------------------" },
	LogLine { active: false, timestamp: 0, line: "--------------------" },
	LogLine { active: false, timestamp: 0, line: "--------------------" },
	LogLine { active: false, timestamp: 0, line: "--------------------" },
	LogLine { active: false, timestamp: 0, line: "--------------------" }
];

//==============================================================================
// Implementations
//==============================================================================
#[allow(dead_code)]
pub fn init() {
	write_line(0);
	write_line(1);
	write_line(2);

	unsafe {
		_LOG_LINES[0].line = "* Debug Initialized ";
	}
}

// pub fn number_to_string(num: &u32) -> &[u8] {
// 	let len: usize = 1;

// 	let tmp_num = num;
// 	let divider = 10;
// 	while (num / divider) > 0 {
// 		divider *= 10;
// 		len += 1;
// 	}
	
// 	let div = 1;

// 	let val = (num / div) % 10;
// 	div += 1;

// 	let buf: &[u8] = &[0, 0, 0, 0, 0];

// 	&mut buf
// }

#[allow(dead_code)]
pub fn push_log(string: &'static str) {
	let index = get_next_log_index();
	unsafe { 
		_LOG_LINES[index].active = true;
		_LOG_LINES[index].timestamp = rtc::get_timestamp();
		_LOG_LINES[index].line = string;
	};
}

fn clear_line(_line_number: usize) {
	
}

fn get_next_log_index() -> usize {
	unsafe { 
		for i in 1.._LOG_LINES.len() {
			if !_LOG_LINES[i].active {
				return i;
			}
		}

		// If all cells are full, empty the oldest and shift all up
		pop_log();
		_LOG_LINES.len() as usize
	}
}

fn pop_log() {
	unsafe { 
		for i in 1..(_LOG_LINES.len() - 1) {
			_LOG_LINES[i].line = _LOG_LINES[i+1].line;
		}
	}
}

fn write_line(line_number: usize) {
	// TODO: use fill_rect funtion to clear this line before writing

	let bytes = unsafe { _LOG_LINES[line_number].line.as_bytes() };
	let len = bytes.len();

	let mut x = DEBUG_INITIAL_X;
	let y = DEBUG_INITIAL_Y + ((line_number as u16) * font::MINIMAL_CHARACTER_HEIGHT * DEBUG_SCALE);

	for i in 0..len {
		write_character(bytes[i] as char, x, y);
		x += font::MINIMAL_CHARACTER_WIDTH * DEBUG_SCALE;
	}
}

fn write_character(c: char, x: u16, y: u16) {
	font::write_minimal_character(c, x, y, DEBUG_FOREGROUND, DEBUG_BACKGROUND, DEBUG_SCALE);
}

//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler(d: &app::DeviceInfo) {
	if d.flags.debug_log_active {
		unsafe {
			let len = _LOG_LINES.len();
			for i in 0..len {
				clear_line(i);

				if _LOG_LINES[i].active {
					write_line(i);
				}
			}
		}
	}
}
