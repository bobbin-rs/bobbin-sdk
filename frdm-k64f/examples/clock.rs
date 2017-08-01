#![no_std]
#![no_main]

#[macro_use]
extern crate frdm_k64f as board;

use board::chip::mcg::MCG;
use board::hal::clock::ClockTree;
// use board::chip::sim::SIM;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Clock Test");
    let clk = board::clock::CLK;
    println!("{:?}", clk);

    println!("Change Bus Divider");
    // println!("CLKDIV1: {:?}", SIM.clkdiv1());
    println!("MCG_S: {:?}", MCG.s());
    board::delay(100);
    board::console::disable();
    MCG.with_c1(|r| r.set_clks(0b10));
    while MCG.s().clkst() != 0b10 {}
    board::console::enable();
    println!("MCG_S: {:?}", MCG.s());
    // println!("CLKDIV1: {:?}", SIM.clkdiv1());
    println!("CORE:           {:?}", clk.core());
    println!("BUS:           {:?}", clk.bus());

    loop {
        board::delay(1000);
        println!("tick");
    }
}
