#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate nucleo_f746zg as board;

use board::prelude::*;

use core::slice;
use board::mcu::flash::*;
use board::bobbin_hal::flash::*;

pub const FLASH_ADDR: *mut u8 = 0x0801_0000 as *mut u8;
pub const FLASH_LEN: usize = 0x100;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let _ = board::init();

    dump(FLASH_ADDR as *const u8, FLASH_LEN);    
    {
        println!("flash erase {:p}", FLASH_ADDR);
        FLASH.erase_begin();
        FLASH.erase(FLASH_ADDR, 0);
        FLASH.erase_end();
        println!("flash erase done")
    }
    dump(FLASH_ADDR as *const u8, FLASH_LEN);    
    {
        println!("Flash write");
        let mut buf = [0u8; 0x100];
        for i in 0..buf.len() {
            buf[i] = i as u8;
        }
        FLASH.write_begin();
        FLASH.write(FLASH_ADDR as *mut u8, &buf).unwrap_or_abort("Error writing flash");
        FLASH.write_end();
        dump(FLASH_ADDR as *const u8, buf.len());    
    }
    println!("done");
    loop {}
}

fn dump(ptr: *const u8, len: usize) {
    let data = unsafe { slice::from_raw_parts(ptr, len) };
    let addr = data.as_ptr() as usize;
    for i in 0..data.len() {
        if i % 32 == 0 {
            if i > 0 {
                println!("");
            }
            print!("{:08x}: ", addr + i);
        }
        if i % 16 == 0 {
            print!(" ");
        }
        if i % 4 == 0 {
            print!(" ");
        }
        print!("{:02x}", data[i]);
    }
    println!("");
}
