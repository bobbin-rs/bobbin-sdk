#![no_std]
#![no_main]

extern crate frdm_k64f as board;

use board::hal::port::GpioPin;
use board::hal::gpio::{DigitalInput, DigitalOutput};

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();    
    let led0 = board::led::LED0.gpio_pin();
    let btn0 = board::btn::BTN0.gpio_pin();        
    loop {
        led0.toggle_output();
        if btn0.input() {
            board::delay(500);
        } else {
            board::delay(100);            
        }        
    }
}
