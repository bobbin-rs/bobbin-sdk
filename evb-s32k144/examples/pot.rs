#![no_std]
#![no_main]

#[macro_use]
extern crate evb_s32k144 as board;

use board::pot::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("POT Test");

    loop {
        println!("{}", POT0.analog_read());
        board::delay(1000);
    }
}
