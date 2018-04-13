#![no_std]
#![no_main]

#[macro_use]
extern crate arduino_zero as board;
extern crate examples;

// use board::common::bits::*;
use board::mcu::pin::*;
use board::mcu::dac::*;

// DAC = PA0 = PA02

#[no_mangle]
pub extern "C" fn main() -> ! {
    let _ = board::init();
    let brd = board::board();
    println!("DAC Test");

    PA02.port().gate_enable();
    PA02.set_dir_input();
    PA02.set_pull_enabled(false);
    PA02.set_pmux_enabled(false);
    PA02.set_pmux_enabled(true).set_pmux(1);

    let dac = DAC;
    dac.gate_enable();
    {
        dac.init();
        dac.enable();
    }
    let dac: DacPeriph = dac.into();

    let mut app = examples::dac::DacExample::new(dac, brd, 5);
    app.run()
}
