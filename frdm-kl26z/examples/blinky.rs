#![no_std]
#![no_main]

extern crate frdm_kl26f as board;

use board::hal::port::GpioPin;
use board::hal::gpio::GpioExt;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let led0 = board::led::LED0.gpio_pin();
    let btn0 = board::btn::BTN0.gpio_pin();
    loop {
        led0.toggle_output();
        //board::delay(100);
        if btn0.input() {
            board::delay(500);
        } else {
            board::delay(250);            
        }        
    }
}
