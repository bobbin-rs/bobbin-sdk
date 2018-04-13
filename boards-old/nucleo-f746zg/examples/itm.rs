#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f746zg as board;

use board::hal::gpio::PinExt;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    board::led::LED0.toggle_output();
    iprintln!("Running ITM Test");
    board::led::LED0.toggle_output();
    let mut i = 0u32;
    loop {
        board::led::LED0.toggle_output();
        iprintln!("Hello, World! {}", i);        
        i = i.wrapping_add(1);
        board::delay(1000);
    }
}
