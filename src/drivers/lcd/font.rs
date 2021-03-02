//==============================================================================
// Notes
//==============================================================================
// drivers::lcd::font.rs
// Minimal and Large Numeric Font
//
// Minimal Characters:
//  Minimal characters are to be used most places. They can be easily scaled. 
//  Each character is based on a 5x8 block. when writing strings, at least 1
//  column of pixels should be used to separate adjacent characters.


//==============================================================================
// Crates and Mods
//==============================================================================
use super::{lcd, lcd_api, st7789};

//==============================================================================
// Enums, Structs, and Types
//==============================================================================\
#[allow(dead_code)]
pub struct MinimalCharacter {
	bytes: [u8; 5]
}

//==============================================================================
// Macros
//==============================================================================


//==============================================================================
// Variables
//==============================================================================
#[allow(dead_code)]
const MINIMAL_CHARACTER_LIST: [MinimalCharacter; 69] = [
	MinimalCharacter { bytes: [ 0x22, 0xA3, 0x1F, 0xC6, 0x20 ] },	// A
	MinimalCharacter { bytes: [ 0xF4, 0x63, 0xE8, 0xC7, 0xC0 ] },	// B
	MinimalCharacter { bytes: [ 0x74, 0x61, 0x08, 0x45, 0xC0 ] },	// C
	MinimalCharacter { bytes: [ 0xF4, 0x63, 0x18, 0xC7, 0xC0 ] },	// D
	MinimalCharacter { bytes: [ 0xFC, 0x21, 0xE8, 0x43, 0xE0 ] },	// E
	MinimalCharacter { bytes: [ 0xFC, 0x21, 0xE8, 0x42, 0x00 ] },	// F
	MinimalCharacter { bytes: [ 0x74, 0x21, 0x38, 0xC5, 0xC0 ] },	// G
	MinimalCharacter { bytes: [ 0x8C, 0x63, 0xF8, 0xC6, 0x20 ] },	// H
	MinimalCharacter { bytes: [ 0xF9, 0x08, 0x42, 0x13, 0xE0 ] },	// I
	MinimalCharacter { bytes: [ 0x08, 0x42, 0x18, 0xC5, 0xC0 ] },	// J
	MinimalCharacter { bytes: [ 0x8C, 0x65, 0xC9, 0x46, 0x20 ] },	// K
	MinimalCharacter { bytes: [ 0x84, 0x21, 0x08, 0x43, 0xE0 ] },	// L
	MinimalCharacter { bytes: [ 0x8E, 0xEB, 0x58, 0xC6, 0x20 ] },	// M
	MinimalCharacter { bytes: [ 0x8E, 0x6B, 0x38, 0xC6, 0x20 ] },	// N
	MinimalCharacter { bytes: [ 0x74, 0x63, 0x18, 0xC5, 0xC0 ] },	// O
	MinimalCharacter { bytes: [ 0xF4, 0x63, 0xE8, 0x42, 0x00 ] },	// P
	MinimalCharacter { bytes: [ 0x74, 0x63, 0x1A, 0xC9, 0xA0 ] },	// Q
	MinimalCharacter { bytes: [ 0xF4, 0x63, 0xE8, 0xC6, 0x20 ] },	// R
	MinimalCharacter { bytes: [ 0x74, 0x60, 0xE0, 0xC5, 0xC0 ] },	// S
	MinimalCharacter { bytes: [ 0xF9, 0x08, 0x42, 0x10, 0x80 ] },	// T
	MinimalCharacter { bytes: [ 0x8C, 0x63, 0x18, 0xC5, 0xC0 ] },	// U
	MinimalCharacter { bytes: [ 0x8C, 0x63, 0x18, 0xA8, 0x80 ] },	// V
	MinimalCharacter { bytes: [ 0xAD, 0x6B, 0x5A, 0xD5, 0x40 ] },	// W
	MinimalCharacter { bytes: [ 0x8C, 0x54, 0x45, 0x46, 0x20 ] },	// X
	MinimalCharacter { bytes: [ 0x8C, 0x62, 0xA2, 0x10, 0x80 ] },	// Y
	MinimalCharacter { bytes: [ 0xF8, 0x44, 0x44, 0x43, 0xE0 ] },	// Z
	
	MinimalCharacter { bytes: [ 0x00, 0x1C, 0x17, 0xC5, 0xE0 ] },	// a
	MinimalCharacter { bytes: [ 0x84, 0x3D, 0x18, 0xC7, 0xC0 ] },	// b
	MinimalCharacter { bytes: [ 0x00, 0x1D, 0x08, 0x41, 0xC0 ] },	// c
	MinimalCharacter { bytes: [ 0x08, 0x5F, 0x18, 0xC5, 0xE0 ] },	// d
	MinimalCharacter { bytes: [ 0x00, 0x1D, 0x1F, 0x41, 0xE0 ] },	// e
	MinimalCharacter { bytes: [ 0x11, 0x1C, 0x42, 0x10, 0x80 ] },	// f
	MinimalCharacter { bytes: [ 0x03, 0x25, 0x27, 0x09, 0x80 ] },	// g
	MinimalCharacter { bytes: [ 0x84, 0x2D, 0x98, 0xC6, 0x20 ] },	// h
	MinimalCharacter { bytes: [ 0x01, 0x00, 0x42, 0x10, 0x80 ] },	// i
	MinimalCharacter { bytes: [ 0x10, 0x04, 0x21, 0x28, 0x80 ] },	// j
	MinimalCharacter { bytes: [ 0x84, 0x25, 0x4C, 0x52, 0x40 ] },	// k
	MinimalCharacter { bytes: [ 0x21, 0x08, 0x42, 0x10, 0x80 ] },	// l
	MinimalCharacter { bytes: [ 0x00, 0x35, 0x5A, 0xD6, 0xA0 ] },	// m
	MinimalCharacter { bytes: [ 0x00, 0x19, 0x29, 0x4A, 0x40 ] },	// n
	MinimalCharacter { bytes: [ 0x00, 0x19, 0x29, 0x49, 0x80 ] },	// o
	MinimalCharacter { bytes: [ 0x03, 0x25, 0x2E, 0x42, 0x00 ] },	// p
	MinimalCharacter { bytes: [ 0x03, 0x25, 0x27, 0x08, 0x60 ] },	// q
	MinimalCharacter { bytes: [ 0x00, 0x2D, 0x88, 0x42, 0x00 ] },	// r
	MinimalCharacter { bytes: [ 0x00, 0x1D, 0x06, 0x0B, 0x80 ] },	// s
	MinimalCharacter { bytes: [ 0x02, 0x3C, 0x84, 0x20, 0xC0 ] },	// t
	MinimalCharacter { bytes: [ 0x00, 0x23, 0x18, 0xC5, 0xE0 ] },	// u
	MinimalCharacter { bytes: [ 0x00, 0x23, 0x18, 0xA8, 0x80 ] },	// v
	MinimalCharacter { bytes: [ 0x00, 0x23, 0x18, 0xD5, 0x40 ] },	// w
	MinimalCharacter { bytes: [ 0x00, 0x22, 0xA2, 0x2A, 0x20 ] },	// x
	MinimalCharacter { bytes: [ 0x00, 0x12, 0x93, 0x84, 0xC0 ] },	// y
	MinimalCharacter { bytes: [ 0x00, 0x3E, 0x22, 0x23, 0xE0 ] },	// z

	MinimalCharacter { bytes: [ 0x23, 0x08, 0x42, 0x11, 0xC0 ] },	// 1
	MinimalCharacter { bytes: [ 0x74, 0x42, 0x64, 0x43, 0xE0 ] },	// 2
	MinimalCharacter { bytes: [ 0x74, 0x42, 0xE0, 0xC5, 0xC0 ] },	// 3
	MinimalCharacter { bytes: [ 0x4A, 0x52, 0xF0, 0x84, 0x20 ] },	// 4
	MinimalCharacter { bytes: [ 0xFC, 0x21, 0xE0, 0xC5, 0xC0 ] },	// 5
	MinimalCharacter { bytes: [ 0x74, 0x21, 0xE8, 0xC5, 0xC0 ] },	// 6
	MinimalCharacter { bytes: [ 0x78, 0x44, 0x42, 0x10, 0x80 ] },	// 7
	MinimalCharacter { bytes: [ 0x74, 0x62, 0xE8, 0xC5, 0xC0 ] },	// 8
	MinimalCharacter { bytes: [ 0x74, 0x62, 0xF0, 0x84, 0x20 ] },	// 9
	MinimalCharacter { bytes: [ 0x74, 0x67, 0x5C, 0xC5, 0xC0 ] },	// 0

	MinimalCharacter { bytes: [ 0x00, 0x00, 0x00, 0x10, 0x00 ] },	// .
	MinimalCharacter { bytes: [ 0x00, 0x00, 0x00, 0x11, 0x00 ] },	// ,
	MinimalCharacter { bytes: [ 0x74, 0x42, 0x22, 0x00, 0x80 ] },	// ?
	MinimalCharacter { bytes: [ 0x21, 0x08, 0x42, 0x00, 0x80 ] },	// !
	MinimalCharacter { bytes: [ 0x00, 0x14, 0x45, 0x00, 0x00 ] },	// *
	MinimalCharacter { bytes: [ 0x00, 0x01, 0xF0, 0x00, 0x00 ] },	// -
	MinimalCharacter { bytes: [ 0x00, 0x08, 0x02, 0x00, 0x00 ] }	// :
];

pub const MINIMAL_CHARACTER_WIDTH: u16 = 5;
pub const MINIMAL_CHARACTER_HEIGHT: u16 = 8;

//==============================================================================
// Implementations
//==============================================================================
#[allow(dead_code)]
fn get_minimal_character(c: char) -> &'static MinimalCharacter {
	match c {
		'A' => &MINIMAL_CHARACTER_LIST[0],
		'B' => &MINIMAL_CHARACTER_LIST[1],
		'C' => &MINIMAL_CHARACTER_LIST[2],
		'D' => &MINIMAL_CHARACTER_LIST[3],
		'E' => &MINIMAL_CHARACTER_LIST[4],
		'F' => &MINIMAL_CHARACTER_LIST[5],
		'G' => &MINIMAL_CHARACTER_LIST[6],
		'H' => &MINIMAL_CHARACTER_LIST[7],
		'I' => &MINIMAL_CHARACTER_LIST[8],
		'J' => &MINIMAL_CHARACTER_LIST[9],
		'K' => &MINIMAL_CHARACTER_LIST[10],
		'L' => &MINIMAL_CHARACTER_LIST[11],
		'M' => &MINIMAL_CHARACTER_LIST[12],
		'N' => &MINIMAL_CHARACTER_LIST[13],
		'O' => &MINIMAL_CHARACTER_LIST[14],
		'P' => &MINIMAL_CHARACTER_LIST[15],
		'Q' => &MINIMAL_CHARACTER_LIST[16],
		'R' => &MINIMAL_CHARACTER_LIST[17],
		'S' => &MINIMAL_CHARACTER_LIST[18],
		'T' => &MINIMAL_CHARACTER_LIST[19],
		'U' => &MINIMAL_CHARACTER_LIST[20],
		'V' => &MINIMAL_CHARACTER_LIST[21],
		'W' => &MINIMAL_CHARACTER_LIST[22],
		'X' => &MINIMAL_CHARACTER_LIST[23],
		'Y' => &MINIMAL_CHARACTER_LIST[24],
		'Z' => &MINIMAL_CHARACTER_LIST[25],
		'a' => &MINIMAL_CHARACTER_LIST[26],
		'b' => &MINIMAL_CHARACTER_LIST[27],
		'c' => &MINIMAL_CHARACTER_LIST[28],
		'd' => &MINIMAL_CHARACTER_LIST[29],
		'e' => &MINIMAL_CHARACTER_LIST[30],
		'f' => &MINIMAL_CHARACTER_LIST[31],
		'g' => &MINIMAL_CHARACTER_LIST[32],
		'h' => &MINIMAL_CHARACTER_LIST[33],
		'i' => &MINIMAL_CHARACTER_LIST[34],
		'j' => &MINIMAL_CHARACTER_LIST[35],
		'k' => &MINIMAL_CHARACTER_LIST[36],
		'l' => &MINIMAL_CHARACTER_LIST[37],
		'm' => &MINIMAL_CHARACTER_LIST[38],
		'n' => &MINIMAL_CHARACTER_LIST[39],
		'o' => &MINIMAL_CHARACTER_LIST[40],
		'p' => &MINIMAL_CHARACTER_LIST[41],
		'q' => &MINIMAL_CHARACTER_LIST[42],
		'r' => &MINIMAL_CHARACTER_LIST[43],
		's' => &MINIMAL_CHARACTER_LIST[44],
		't' => &MINIMAL_CHARACTER_LIST[45],
		'u' => &MINIMAL_CHARACTER_LIST[46],
		'v' => &MINIMAL_CHARACTER_LIST[47],
		'w' => &MINIMAL_CHARACTER_LIST[48],
		'x' => &MINIMAL_CHARACTER_LIST[49],
		'y' => &MINIMAL_CHARACTER_LIST[50],
		'z' => &MINIMAL_CHARACTER_LIST[51],
		'1' => &MINIMAL_CHARACTER_LIST[52],
		'2' => &MINIMAL_CHARACTER_LIST[53],
		'3' => &MINIMAL_CHARACTER_LIST[54],
		'4' => &MINIMAL_CHARACTER_LIST[55],
		'5' => &MINIMAL_CHARACTER_LIST[56],
		'6' => &MINIMAL_CHARACTER_LIST[57],
		'7' => &MINIMAL_CHARACTER_LIST[58],
		'8' => &MINIMAL_CHARACTER_LIST[59],
		'9' => &MINIMAL_CHARACTER_LIST[60],
		'0' => &MINIMAL_CHARACTER_LIST[61],
		'.' => &MINIMAL_CHARACTER_LIST[62],
		',' => &MINIMAL_CHARACTER_LIST[63],
		'?' => &MINIMAL_CHARACTER_LIST[64],
		'!' => &MINIMAL_CHARACTER_LIST[65],
		'*' => &MINIMAL_CHARACTER_LIST[66],
		'-' => &MINIMAL_CHARACTER_LIST[67],
		':' => &MINIMAL_CHARACTER_LIST[68],
		_ => &MinimalCharacter { bytes: [0x00, 0x00, 0x00, 0x00, 0x00] }
	}
}

#[allow(dead_code)]
pub fn write_minimal_character(p: &nrf52832_pac::Peripherals, c: char, x: u16, y: u16, fg: u16, bg: u16, scale: u16) {
	let char_width = MINIMAL_CHARACTER_WIDTH * scale;
	let char_height = MINIMAL_CHARACTER_HEIGHT * scale;

	lcd_api::set_window(p, x, char_width, y, char_height);
	lcd::write_command(p, st7789::COMMAND::MEMORY_WRITE);
	let bytes = get_minimal_character(c).bytes;
	let bg_color: [u8; 2] = [ ((bg & 0xFF00) >> 8) as u8, (bg & 0x00FF) as u8 ];
	let fg_color: [u8; 2] = [ ((fg & 0xFF00) >> 8) as u8, (fg & 0x00FF) as u8 ];

	let mut bit_count: usize = 0;
	let mut byte_count: usize = 0;


	for _row in 0..8 {
		let mut tmp_bit_count: usize = bit_count;
		let mut tmp_byte_count: usize = byte_count;

		for _row_scaler in 0..scale {
			tmp_bit_count = bit_count;
			tmp_byte_count = byte_count;

			for _col in 0..5 {
				let pixel_is_on: bool = (bytes[tmp_byte_count] & (0x80 >> tmp_bit_count)) > 0;

				for _col_scaler in 0..scale {
					if pixel_is_on {
						lcd::write_data(p, &fg_color);
					}
					else {
						lcd::write_data(p, &bg_color);
					}
				}
				
				tmp_bit_count += 1;
				if tmp_bit_count == 8 {
					tmp_bit_count = 0;
					tmp_byte_count += 1;
				}
			}
		}

		bit_count = tmp_bit_count;
		byte_count = tmp_byte_count;

	}
}

//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================






































































