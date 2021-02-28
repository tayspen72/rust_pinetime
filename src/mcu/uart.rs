//==============================================================================
// Notes
//==============================================================================
// mcu::uart.rs

//==============================================================================
// Crates and Mods
//==============================================================================
// use nrf52832_pac::{interrupt, uart0};
// use crate::config;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
// #[allow(dead_code)]
// pub struct UartLine {
// 	pub cts_pin: Option<u8>,
// 	pub rts_pin: Option<u8>,
// 	pub rx_pin: u8,
// 	pub tx_pin: u8,
// 	pub baud: uart0::baudrate::BAUDRATE_A,
// 	pub parity: uart0::config::PARITY_A,
// 	pub echo_enabled: bool,
// }

//==============================================================================
// Macros
//==============================================================================


//==============================================================================
// Variables
//==============================================================================
// static mut _UART_RX_BUFFER: [u8; config::UART_RX_BUFFER_LENGTH] = [ 0; config::UART_RX_BUFFER_LENGTH];
// static mut _UART_BUFFER_HEAD: usize = 0;
// static mut _UART_BUFFER_TAIL: usize = 0;

//==============================================================================
// Implementations
//==============================================================================
// #[allow(dead_code)]
// pub fn init(_p: &nrf52832_pac::Peripherals, _uartline: &UartLine) {
// 	let uart = &p.UART0;

// 	nrf52832_pac::NVIC::mask(nrf52832_pac::Interrupt::UARTE0_UART0);

// 	uart.enable.write(|w| w.enable().disabled());

// 	if (uartline.cts_pin == None) || (uartline.rts_pin == None) {
// 		uart.config.write(|w| w.hwfc().disabled());
// 	}
// 	else {
// 		uart.pselcts.write(|w| unsafe { w.bits(uartline.cts_pin.unwrap() as u32) });
// 		uart.pselrts.write(|w| unsafe { w.bits(uartline.rts_pin.unwrap() as u32) });
// 		uart.config.write(|w| w.hwfc().enabled());
// 	}

// 	uart.pselrxd.write(|w| unsafe { w.bits(uartline.rx_pin as u32) });
// 	uart.pseltxd.write(|w| unsafe { w.bits(uartline.tx_pin as u32) });

// 	uart.config.modify(|_, w| w.parity().variant(uartline.parity));

// 	uart.baudrate.write(|w| w.baudrate().variant(uartline.baud ));

// 	uart.intenset.write(|w| w.rxdrdy().set());
// 	uart.events_rxdrdy.write(|w| unsafe { w.bits(1) });

// 	uart.enable.write(|w| w.enable().enabled());

// 	unsafe { nrf52832_pac::NVIC::unmask(nrf52832_pac::Interrupt::UARTE0_UART0); }
// }

// #[allow(dead_code)]
// pub fn tx(p: &nrf52832_pac::Peripherals, byte: u8) {
// 	let uart = &p.UART0;

// 	uart.tasks_starttx.write(|w| unsafe { w.bits(1) });
// 	uart.txd.write(|w| unsafe { w.txd().bits(byte) });
	
// 	while uart.events_txdrdy.read().bits() == 0 {};
// 	uart.events_txdrdy.write(|w| unsafe { w.bits(0) });

// 	uart.tasks_stoptx.write(|w| unsafe { w.bits(1) });
// }

//==============================================================================
// Interrupt Handler
//==============================================================================
// #[interrupt]
//  fn UARTE0_UART0() {
// 	unsafe { 
// 		let uart = nrf52832_pac::Peripherals::steal().UART0;

// 		if uart.events_rxdrdy.read().bits() > 0 {
// 			uart.events_rxdrdy.write(|w| w.bits(0));
// 			_UART_RX_BUFFER[_UART_BUFFER_TAIL] = uart.rxd.read().bits() as u8;
// 			if _UART_BUFFER_TAIL == (config::UART_RX_BUFFER_LENGTH - 1) {
// 				_UART_BUFFER_TAIL = 0;
// 			}
// 		}
// 		else if uart.events_error.read().bits() > 0 {
// 			uart.events_rxdrdy.write(|w| w.bits(0));
// 		}
// 	}
// }

//==============================================================================
// Task Handler
//==============================================================================
// pub fn task_handler() {
// 	let mut tmp_tail: usize = 0;

// 	cortex_m::interrupt::free(|_| {
// 		unsafe { 
// 			if _UART_BUFFER_HEAD == _UART_BUFFER_TAIL {
// 				return;
// 			}

// 			tmp_tail = _UART_BUFFER_TAIL;
// 		}
// 	});

// 	while unsafe { _UART_BUFFER_HEAD != tmp_tail } {
// 		// Do something with the data

// 		unsafe { 
// 			_UART_BUFFER_HEAD += 1;
// 			if _UART_BUFFER_HEAD == (config::UART_RX_BUFFER_LENGTH - 1) {
// 				_UART_BUFFER_HEAD = 0;
// 			}
// 		}
// 	}
// }