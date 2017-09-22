#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]

extern crate silabs_slstk3401a as board;

use board::led::*;
use board::btn::*;
use board::hal::gpio::*;

// LED0 = PF4
// LED1 = PF5
// BTN0 = PF6
// BTN1 = PF7

pub const PIN: usize = 4;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();


    loop {
        LED0.toggle_output();
        if BTN0.input() {
            board::delay(100)            
        } else {
            board::delay(1000);
        }
    }
}
