#![no_std]
#![no_main]

#[macro_use]
extern crate discovery_stm32f429i as board;
extern crate examples;

use board::mcu::pin::*;
use board::mcu::dac::*;
use board::prelude::*;

// DAC_CH2 = DAC_OUT2 = PA4 or PA5

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init().run(|brd| {
        println!("DAC Test");

        let dac = DAC;
        let dac_ch = DAC_CH2;
        let dac_pin = PA5;

        dac.gate_enable();
        dac_pin.port().gate_enable();
        dac_pin.mode_analog();
        dac_pin.connect_to(dac_ch);

        dac_ch.enable();

        let dac_ch: DacCh = dac_ch.into();
        let mut app = examples::dac::DacExample::new(dac_ch, brd.tick(), 5);

        app.run()
    })
}
