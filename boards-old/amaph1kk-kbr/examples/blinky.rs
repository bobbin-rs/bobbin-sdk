#![no_std]
#![no_main]

extern crate amaph1kk_kbr as board;

use board::hal::gpio::*;
use board::led::*;
use board::btn::{BTN2, DigitalInput};


// LED0 = GP17
// BTN2 = GP16

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();

    let leds: [GpioPin; 5] = [
        LED0.into(),
        LED1.into(),
        LED2.into(),
        LED3.into(),
        LED4.into(),
    ];
    loop {
        for c in 0..5 {
            for i in 0..5 {
                leds[i].set_output(c == i);
            }
            if BTN2.input() {
                board::delay(100);            
            } else {
                board::delay(1000);
            }
        }
    }    
}
