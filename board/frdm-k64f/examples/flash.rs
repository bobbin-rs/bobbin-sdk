#![no_std]
#![no_main]

extern crate frdm_k64f as board;
extern crate examples;

pub const FLASH_ADDR: *mut u8 = 0x008_0000 as *mut u8;
pub const FLASH_LEN: usize = 0x100;

#[no_mangle]
pub extern "C" fn main() -> ! {
    examples::flash::run(board::init(), FLASH_ADDR, FLASH_LEN)
}