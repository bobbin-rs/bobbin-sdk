#![no_std]
#![no_main]

extern crate can_cam as board;

use board::led::*;
use board::btn::*;


#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    loop {
        LED0.toggle_output();
        if BTN0.input() {
            board::delay(100);
        } else {
            board::delay(1000);
            
        }
    }
}
