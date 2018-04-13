#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate nucleo_l432kc as board;

extern "C" {
    static mut _stext: u32;
}

use core::slice;
use board::mcu::flash::*;

pub const FLASH_ADDR: *mut u32 = 0x0801_0000 as *mut u32;
pub const FLASH_LEN: usize = 0x100;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let _ = board::init();

    println!("Flash Test");
    {
        println!("flash_locked: {}, unlocking", FLASH.flash_locked());
        FLASH.flash_unlock();
        println!("flash_locked: {}, locking", FLASH.flash_locked());
        FLASH.flash_lock();
        println!("flash_locked: {}", FLASH.flash_locked());
    }
    dump(FLASH_ADDR as *const u8, FLASH_LEN);    
    {
        println!("flash erase {:p}", FLASH_ADDR);
        FLASH.unlocked(|f| f.flash_erase(FLASH_ADDR));
        println!("flash erase done")
    }
    dump(FLASH_ADDR as *const u8, FLASH_LEN);    
    {
        println!("Flash write");
        let mut buf = [0u32; 0x100 / 4];
        for i in 0..buf.len() {
            buf[i] = i as u32;
        }
        FLASH.unlocked(|f| {
            f.flash_write(FLASH_ADDR as *mut u32, &buf);
        });
        dump(FLASH_ADDR as *const u8, buf.len() * 4);    
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

pub trait FlashLockUnlock {
    fn flash_locked(&self) -> bool;
    fn flash_unlock(&self);
    fn flash_lock(&self);
    fn unlocked<T, F: FnOnce(&Self)->T>(&self, f: F) -> T {
        self.flash_unlock();
        let ret = f(self);
        self.flash_lock();
        ret
    }
}

pub trait FlashBusy {
    fn flash_busy(&self) -> bool;
}

pub trait FlashErase {
    fn flash_erase(&self, addr: *const u32);
}

pub trait FlashWrite<T> {
    fn flash_write(&self, addr: *mut T, data: &[T]) -> usize;
}

pub const KEY1: u32 = 0x45670123;
pub const KEY2: u32 = 0xCDEF89AB;

impl FlashLockUnlock for FlashPeriph {
    fn flash_locked(&self) -> bool {
        self.cr().test_lock()
    }

    fn flash_lock(&self) {
        self.with_cr(|r| r.set_lock(1));
    }

    fn flash_unlock(&self) {
        self.set_keyr(|r| r.set_keyr(KEY1));
        self.set_keyr(|r| r.set_keyr(KEY2));
    }
}

impl FlashBusy for FlashPeriph {
    fn flash_busy(&self) -> bool {
        self.sr().test_bsy()
    }
}

impl FlashErase for FlashPeriph {    
    fn flash_erase(&self, addr: *const u32) {
        while self.flash_busy() {}
        let addr = addr as u32;
        let pnb = if addr & 0x7ff == 0 && addr >= 0x0800_0000 && addr <= 0x0803_F800 {
            (addr >> 11) as u8
        } else {
            panic!("Invalid flash page address")
        };
        self.with_cr(|r| r.set_pnb(pnb).set_per(1).set_start(1));
        while self.flash_busy() {}
        self.with_cr(|r| r.set_per(0));
    }
}


impl FlashWrite<u32> for FlashPeriph {
    fn flash_write(&self, addr: *mut u32, data: &[u32]) -> usize {
        while self.flash_busy() {}
        self.with_cr(|r| r.set_pg(1));
        let mut i = 0;
        while i < data.len() {
            unsafe {
                *addr.offset(i as isize) = data[i];
                i += 1;
                *addr.offset(i as isize) = data[i];
                i += 1;
            }
            while self.flash_busy() {}
        }
        self.with_cr(|r| r.set_pg(0));
        data.len()
    }
}

