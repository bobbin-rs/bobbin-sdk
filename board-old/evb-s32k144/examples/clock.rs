#![no_std]
#![no_main]

#[macro_use]
extern crate evb_s32k144 as board;

use board::clock::CLK;
use board::hal::clock::Clock;
use board::chip::lpuart::*;
use board::chip::lpit::*;
use board::chip::scg::SCG;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Running Clock");
    println!("{:?}", CLK);
    println!("LPUART0: {:?}", LPUART0.clock(&CLK));    
    println!("LPUART1: {:?}", LPUART1.clock(&CLK));
    println!("LPUART2: {:?}", LPUART2.clock(&CLK));
    println!("LPIT0: {:?}", LPIT0.clock(&CLK));
    loop {
        println!("Switching SPLLDIV2 to 80MHz");
        board::delay(10);
        board::console::disable();
        SCG.with_splldiv(|r| r.set_splldiv2(0b010));

        board::console::reinit();
        println!("Running at 80 MHz");
        board::delay(1000);

        println!("Switching SPLLDIV2 to 40MHz");
        board::delay(10);
        board::console::disable();
        SCG.with_splldiv(|r| r.set_splldiv2(0b011));
        
        board::console::reinit();
        println!("Running at 40 MHz");
        board::delay(1000);
    }

}
