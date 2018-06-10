#![no_std]
#![no_main]

extern crate discovery_stm32f429i as board;
extern crate examples;

#[no_mangle]
pub extern "C" fn main() -> ! {
    examples::tick_handler::run_with_sys(board::init())
}
