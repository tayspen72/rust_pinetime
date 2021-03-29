//==============================================================================
// Notes
//==============================================================================
// mcu::adc.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use core::cell::RefCell;
use core::ops::DerefMut;
use core::ptr;
use cortex_m::interrupt::{free, Mutex};
use crate::config;
use nrf52832_pac;
use super::gpio;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[derive(Copy, Clone)]
pub struct AdcLine {
	pub pin: u8,
	pub channel: u8,
	pub resolution: nrf52832_pac::saadc::resolution::VAL_A
}

//==============================================================================
// Variables
//==============================================================================
static ADC_HANDLE: Mutex<RefCell<Option<nrf52832_pac::SAADC>>> = 
	Mutex::new(RefCell::new(None));

const ADC_LINE: AdcLine = AdcLine {
	pin: config::BATTERY_ADC_PIN,
	channel: config::BATTERY_ADC_CHANNEL,
	resolution: nrf52832_pac::saadc::resolution::VAL_A::_12BIT
};

//==============================================================================
// Public Functions
//==============================================================================
pub fn init(adc: nrf52832_pac::SAADC){
	configure(&adc);

	free(|cs| ADC_HANDLE.borrow(cs).replace(Some(adc)));
}

pub fn get_busy() -> bool {
	// Will always be a blocking call. Return false
	false
}

pub fn read_adc() -> u16 {
	free(|cs| {
		if let Some(ref mut adc) = ADC_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			adc.ch[ADC_LINE.channel as usize].config.write(|w| w
				.resp().bypass()
				.resn().bypass()
				.gain().gain1_6()
				.refsel().internal()
				.tacq()._10us()
				.mode().se()
				.burst().disabled()
			);

			for s in 0..config::ADC_RAM_BUFFER_LEN {
				// Define the result pointer and number of tranfers
				adc.result.ptr.write(|w| unsafe { w.bits(config::ADC_RAM_BUFFER + (s * 2) as u32)});
				adc.result.maxcnt.write(|w| unsafe { w.maxcnt().bits(1) });

				// Clear sampling flags
				adc.events_done.write(|w| unsafe { w.bits(0) });
				// adc.events_resultdone.write(|w| unsafe { w.bits(0) });
				// adc.events_end.write(|w| unsafe { w.bits(0) });

				// Start the one-shot read
				// adc.tasks_start.write(|w| unsafe { w.bits(1) });
				adc.tasks_sample.write(|w| unsafe { w.bits(1) });

				// Wait for finish
				while adc.events_done.read().bits() == 0 {}
			}

			let mut sample: u32 = 0;
			for s in 0..config::ADC_RAM_BUFFER_LEN {
				let address: *const u16 = (config::ADC_RAM_BUFFER as u32 + (s as u32 * 2)) as *const u16;
				sample = sample + unsafe { *ptr::read(ptr::addr_of!(address)) as u32 };
			}
			(sample / config::ADC_RAM_BUFFER_LEN as u32) as u16
		}
		else {
			0
		}
	})
}

//==============================================================================
// Private Functions
//==============================================================================
fn configure(adc: &nrf52832_pac::SAADC) {
	adc.enable.write(|w| w.enable().disabled());
	
	// Configure the pin
	gpio::pin_setup(
		ADC_LINE.pin,
		nrf52832_pac::p0::pin_cnf::DIR_A::INPUT, 
		gpio::PinState::PinLow,
		nrf52832_pac::p0::pin_cnf::PULL_A::DISABLED
	);

	// Clear all bits to be safe
	adc.intenclr.write(|w| unsafe { w.bits(0x3FFFFF) });

	// Enable end event interrupt
	adc.intenset.write(|w| w.end().set());

	// Enable the ADC when finished
	adc.enable.write(|w| w.enable().enabled());
}

//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
