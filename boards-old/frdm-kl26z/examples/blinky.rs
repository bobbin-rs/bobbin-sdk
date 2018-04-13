#![no_std]
#![no_main]

extern crate frdm_kl26z as board;

use board::led::*;
use board::btn::*;

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
