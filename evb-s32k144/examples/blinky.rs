#![no_std]
#![no_main]
#![feature(asm)]

extern crate evb_s32k144 as board;

use board::led::*;
use board::btn::*;
use board::hal::gpio::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let btn0 = BTN0;
    let led0 = LED0;
    let led1 = LED1;
    let led2 = LED2;

    // LEDs are active LOW

    loop {
        led0.toggle_output();
        led1.toggle_output();
        led2.toggle_output();
        if btn0.input() {
            board::delay(100);
        } else {
            board::delay(500);
        }
    }
}
