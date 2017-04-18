#![no_std]
#![no_main]
#![feature(asm)]

//#[macro_use]
extern crate arduino_zero as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let led0 = board::led::led0();
    loop {
        led0.set(true);
        for _ in 0..10_000_000 { unsafe { asm!("nop")}}
        
        led0.set(false);
        for _ in 0..10_000_000 { unsafe { asm!("nop")}}
    }
}
