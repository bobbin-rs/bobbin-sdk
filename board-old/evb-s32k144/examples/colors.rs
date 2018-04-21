#![no_std]
#![no_main]
#![feature(asm)]

extern crate evb_s32k144 as board;

use board::btn::*;
use board::led::*;


#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();

    // LEDs are active LOW

    LED0.set_output(true);
    LED1.set_output(true);
    LED2.set_output(true);

    let mut n = 0u8;

    loop {
        LED0.set_output(n & 1 != 0);
        LED1.set_output(n & 2 != 0);
        LED2.set_output(n & 4 != 0);
        if BTN0.input() {
            board::delay(100);
        } else {
            board::delay(500);
        }
        n = n.wrapping_add(1);
    }
}
