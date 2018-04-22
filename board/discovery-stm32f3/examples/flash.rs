#![no_std]
#![no_main]

extern crate discovery_stm32f3 as board;
extern crate examples;

pub const FLASH_ADDR: *mut u8 = 0x0800_4000 as *mut u8;
pub const FLASH_LEN: usize = 0x100;

#[no_mangle]
pub extern "C" fn main() -> ! {
    examples::flash::run(board::init(), FLASH_ADDR, FLASH_LEN)
}