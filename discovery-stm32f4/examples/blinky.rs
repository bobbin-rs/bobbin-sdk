#![no_std]
#![no_main]

extern crate discovery_stm32f4 as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let led0 = board::led::led0();
    loop {
        led0.toggle();
        board::delay(500);
    }
}
