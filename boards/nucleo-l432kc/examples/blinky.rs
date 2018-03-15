#![no_std]
#![no_main]

extern crate nucleo_l432kc as board;

use board::common::digital::*;
use board::led::LED0;
use board::btn::BTN0;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    
    loop {
        LED0.toggle_output();
        if BTN0.input() {            
            board::delay(500);
        } else {
            board::delay(100);
        }
    }
}

