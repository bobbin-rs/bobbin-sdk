#![no_std]
#![no_main]

#[macro_use]
extern crate arduino_zero as board;

use board::clock::*;
use board::mcu::tc::*;
use board::mcu::sercom::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let _ = board::init();

    println!("Sources");
    println!("---");
    for i in 0..9 {
        println!("0x{:02x}: {}", i, Clk::src(i.into()).as_u32());
    }
    println!("");
    println!("Clock Generators");
    println!("---");
    for i in 0..9 {
        println!("GENCLK_{}: {:?} {:?} {}", 
            i, 
            Clk::read_genctrl(i.into()), 
            Clk::read_gendiv(i.into()), 
            Clk::gclkgen(i.into()).as_u32()
        );
    }
    println!("");

    println!("Clocks");
    println!("---");
    for i in 0..0x27 {
        println!("CLK_{}: {:?} {}", i, Clk::read_clkctrl(i.into()), Clk::gclk(i.into()).as_u32());
    }
    println!("");

    println!("TC3: {}", Tree::clock_for(TC3).as_u32());
    println!("SERCOM5: {}", Tree::clock_for(SERCOM5).as_u32());


    loop {}
}
