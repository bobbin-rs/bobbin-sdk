#![no_std]
#![no_main]

extern crate amaph1kk_kbr as board;

use board::led::{LED0, DigitalOutput};
use board::btn::{BTN2, DigitalInput};


// LED0 = GP17
// BTN2 = GP16

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();

    loop {
        LED0.toggle_output();
        if BTN2.input() {
            board::delay(1000);
        } else {
            board::delay(100);            
        }
    }    
}
