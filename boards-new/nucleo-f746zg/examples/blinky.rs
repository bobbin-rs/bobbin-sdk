#![no_std]
#![no_main]
#![feature(asm)]

extern crate nucleo_f746zg as board;

use board::led::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();    
    let brd = board::board();

    loop {
        for i in 0..brd.get_led_count() {
            brd.get_led(i).on();
            board::delay(500);
        }
        for i in 0..brd.get_led_count() {
            brd.get_led(i).off();
            board::delay(500);
        }
    }
}