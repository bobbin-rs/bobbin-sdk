#![no_std]
#![no_main]

#[macro_use]
extern crate ek_tm4c1294xl as board;

use board::hal::gpio::GpioExt;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    
    iprintln!("Running Console");    
    let mut i = 0u32;
    loop {
        board::led::LED0.toggle_output();
        iprintln!("Hello, World! {}", i);
        i = i.wrapping_add(1);
        board::delay(1000);
    }
}
