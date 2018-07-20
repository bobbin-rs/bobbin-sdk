#![no_std]
#![no_main]

extern crate feather_m0 as board;
extern crate examples;

#[no_mangle]
pub extern "C" fn main() -> ! {
    examples::pend_handler::run_with_sys(board::init())
}
