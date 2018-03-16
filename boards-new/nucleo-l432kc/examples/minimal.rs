#![no_std]
#![no_main]

extern crate nucleo_l432kc as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    loop {}
}