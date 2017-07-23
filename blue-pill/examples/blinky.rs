#![no_std]
#![no_main]

extern crate blue_pill as board;
use board::hal::gpio::PinExt;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let led0 = board::led::LED0;
    loop {
        led0.toggle_output();
        board::delay(500);
    }
}
