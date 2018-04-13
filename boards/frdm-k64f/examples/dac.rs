#![no_std]
#![no_main]

#[macro_use]
extern crate frdm_k64f as board;
extern crate examples;

use board::mcu::pin::*;
use board::mcu::dac::*;
// DAC_CH2 = DAC_OUT2 = PA5 = D13

#[no_mangle]
pub extern "C" fn main() -> ! {
    let _ = board::init();
    let brd = board::board();
    println!("DAC Test");

    let dac = DAC0;

    dac.gate_enable();
    dac.with_c0(|r| r.set_dacen(1).set_dacrfs(1));
    let dac: DacPeriph = dac.into();
    let mut app = examples::dac::DacExample::new(dac, brd, 5);
    app.run()
}
