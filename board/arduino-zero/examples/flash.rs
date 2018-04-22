#![no_std]
#![no_main]

extern crate arduino_zero as board;

use board::prelude::*;
use board::mcu::nvmctrl::*;

pub const FLASH_ADDR: *mut u8 = 0x0003_0000 as *mut u8;
pub const FLASH_LEN: usize = 0x100;

// PSZ = 0x03 -> 64 byte pages
// NVMP = 0x1000 -> 4096 pages
// Total Memory - 262,144 bytes (256k)

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut brd = board::init();
    brd.run(|brd| {
        if let Some(console) = brd.console() {
            console.writeln(b"Erasing Flash");

            NVMCTRL.erase_begin();
            NVMCTRL.erase(FLASH_ADDR).unwrap_or_abort("Error erasing flash");
            NVMCTRL.erase_end();
            unsafe { console.dump_ptr(FLASH_ADDR as *const u8, FLASH_LEN); }

            console.writeln(b"Writing Flash");
            let mut buf = [0u8; 0x100];
            for i in 0..buf.len() {
                buf[i] = i as u8;
            }
            NVMCTRL.write_begin();
            NVMCTRL.write(FLASH_ADDR as *mut u8, &buf).unwrap_or_abort("Error writing flash");
            NVMCTRL.write_end();
            unsafe { console.dump_ptr(FLASH_ADDR as *const u8, FLASH_LEN); }
            console.writeln(b"Done");    
        }
        loop {}
    })
}