#![no_std]
#![no_main]

extern crate teensy_36 as board;

use board::hal::port::GpioPin;
use board::hal::gpio::DigitalOutput;

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
