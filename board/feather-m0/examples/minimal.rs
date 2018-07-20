#![no_std]
#![no_main]

extern crate feather_m0 as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let _ = board::init();
    loop {}
}