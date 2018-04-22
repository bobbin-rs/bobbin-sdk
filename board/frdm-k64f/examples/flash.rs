#![no_std]
#![no_main]

extern crate frdm_k64f as board;

use board::prelude::*;
use board::mcu::ftfe::*;

pub const FLASH_ADDR: *mut u8 = 0x008_0000 as *mut u8;
pub const FLASH_LEN: usize = 0x100;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut brd = board::init();
    brd.run(|brd| {
        if let Some(console) = brd.console() {
            console.writeln(b"Erasing Flash");

            FTFE.erase_begin();
            FTFE.erase(FLASH_ADDR).unwrap_or_abort("Error erasing flash");
            FTFE.erase_end();
            
            unsafe { console.dump_ptr(FLASH_ADDR as *const u8, FLASH_LEN); }

            console.writeln(b"Writing Flash");
            let mut buf = [0u8; 0x100];
            for i in 0..buf.len() {
                buf[i] = i as u8;
            }
            FTFE.write_begin();
            FTFE.write(FLASH_ADDR as *mut u8, &buf).unwrap_or_abort("Error writing flash");
            FTFE.write_end();
            unsafe { console.dump_ptr(FLASH_ADDR as *const u8, FLASH_LEN); }
            console.writeln(b"Done");    
        }
        loop {}
    })
}