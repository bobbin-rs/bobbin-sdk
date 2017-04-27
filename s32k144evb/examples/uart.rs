#![no_std]
#![no_main]

extern crate s32k144evb as board;

use core::fmt::Write;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let mut u = board::serial::serial1();
    let led0 = board::led::led_red();
    let mut i = 0;
    loop {
        led0.toggle();
        write!(u, "Hello, World: {}\r\n", i).unwrap();
        
        i += 1;
        board::delay(1000);
    }
}

