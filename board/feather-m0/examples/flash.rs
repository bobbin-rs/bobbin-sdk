#![no_std]
#![no_main]

extern crate feather_m0 as board;
extern crate examples;

pub const FLASH_ADDR: *mut u8 = 0x0003_0000 as *mut u8;
pub const FLASH_LEN: usize = 0x100;

// PSZ = 0x03 -> 64 byte pages
// NVMP = 0x1000 -> 4096 pages
// Total Memory - 262,144 bytes (256k)

#[no_mangle]
pub extern "C" fn main() -> ! {
    examples::flash::run(board::init(), FLASH_ADDR, FLASH_LEN)
}