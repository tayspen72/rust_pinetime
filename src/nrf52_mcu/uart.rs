use super::super::nrf52_mcu as mcu;
// use super::uart as uart;

#[allow(dead_code)]
pub fn get_state() -> mcu::PeripheralState 
{
    // match SPI_STATE{
    //     mcu::PeripheralState ::Fault => mcu::PeripheralState ::Fault,
    //     mcu::PeripheralState ::Ready => mcu::PeripheralState ::Ready,
    //     mcu::PeripheralState ::Uninitialized => mcu::PeripheralState ::Uninitialized
    // }
    mcu::PeripheralState::Fault
}

#[allow(dead_code)]
pub fn init_uart(_p: &nrf52832_pac::Peripherals){
    
}