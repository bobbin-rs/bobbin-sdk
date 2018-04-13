#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_l432kc as board;
extern crate examples;

use board::mcu::pin::*;
use board::mcu::dac::*;

// DAC1_CH1 = DAC1_OUT1 = PA4 = A3

#[no_mangle]
pub extern "C" fn main() -> ! {
    let _ = board::init();
    let brd = board::board();
    println!("DAC Test");

    let dac = DAC1;
    let dac_ch = DAC1_CH1;
    let dac_pin = PA4;

    dac.gate_enable();
    dac_pin.port().gate_enable();
    dac_pin.mode_analog();
    dac_pin.connect_to(dac_ch);
    
    dac_ch.enable();    

    let dac_ch: DacCh = dac_ch.into();
    let mut app = examples::dac::DacExample::new(dac_ch, brd, 5);
    app.run()

}
