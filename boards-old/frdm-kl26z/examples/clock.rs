#![no_std]
#![no_main]

#[macro_use]
extern crate frdm_kl26z as board;

use board::hal::clock::{ClockTree, Clock};
use board::clock::CLK;
use board::chip::mcg::MCG;
use board::chip::uart0::UART0;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    board::delay(100);
    println!("Clock");        
    println!("Core: {}", CLK.coreclk().unwrap());
    println!("Bus: {}", CLK.busclk().unwrap());
    println!("FLL: {}", CLK.mcgfllclk().unwrap());    
    println!("UART0: {}", UART0.clock(&CLK).unwrap());
    loop {
        println!("Switching to OSCCLK");
        board::delay(10);
        board::console::disable();
        MCG.with_c1(|r| r.set_clks(0b10));
        while MCG.s().clkst() != 0b10 {}
        board::console::enable();
        println!("Running on OSCCLK");
        board::delay(1000);

        println!("Switching to PLL");
        board::delay(10);
        board::console::disable();
        MCG.with_c1(|r| r.set_clks(0b00));
        while MCG.s().clkst() != 0b11 {}
        board::console::enable();
        println!("Running on PLL");
        board::delay(1000);
    }
}
