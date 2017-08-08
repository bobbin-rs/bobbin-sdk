#![no_std]
#![no_main]
#![feature(asm)]
extern crate frdm_k64f as board;

use board::hal::port::GpioPin;
use board::hal::gpio::GpioExt;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::led::init();    
    let led0 = board::led::LED0.gpio_pin();
    loop {
        led0.toggle_output();
        for _ in 0..2_000_000 {
            unsafe { asm!("nop"); }
        }
    }
}
