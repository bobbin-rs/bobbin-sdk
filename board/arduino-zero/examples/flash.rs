#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate arduino_zero as board;

extern "C" {
    static mut _stext: u32;
}

use board::mcu::nvmctrl::*;

use core::slice;

pub const FLASH_ADDR: *mut u32 = 0x0003_0000 as *mut u32;
pub const FLASH_LEN: usize = 0x100;

// PSZ = 0x03 -> 64 byte pages
// NVMP = 0x1000 -> 4096 pages
// Total Memory - 262,144 bytes (256k)


#[no_mangle]
pub extern "C" fn main() -> ! {
    let _ = board::init();

    println!("Flash Test");
    println!("PARAM: {:?}", NVMCTRL.param());
    println!("LOCK: {:?}", NVMCTRL.lock());


    dump(FLASH_ADDR as *const u8, FLASH_LEN);    
    {
        println!("flash erase {:p}", FLASH_ADDR);
        NVMCTRL.flash_erase(FLASH_ADDR);
        println!("flash erase done")
    }
    dump(FLASH_ADDR as *const u8, FLASH_LEN);    
    {
        println!("Flash write");
        let mut buf = [0u32; 0x100 / 4];
        for i in 0..buf.len() {
            buf[i] = i as u32;
        }
        NVMCTRL.flash_write(FLASH_ADDR as *mut u32, &buf);
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

impl FlashErase for NvmctrlPeriph {    
    fn flash_erase(&self, addr: *const u32) {
        // Note from https://github.com/blacksphere/blackmagic/blob/master/src/target/samd.c
        /* Write address of first word in row to erase it */
		/* Must be shifted right for 16-bit address, see Datasheet §20.8.8 Address */
        self.set_addr(|_| Addr(addr as u32 >> 1));
        self.set_ctrla(|r| r.set_cmdex(0xa5).set_cmd(0x41));
        self.set_ctrla(|r| r.set_cmdex(0xa5).set_cmd(0x02));
        while !self.intflag().test_ready() {}
        // self.set_ctrla(|r| r.set_cmdex(0xa5).set_cmd(0x46));
    }
}


impl FlashWrite<u32> for NvmctrlPeriph {
    fn flash_write(&self, addr: *mut u32, data: &[u32]) -> usize {
        for i in 0..data.len() {
            unsafe { *addr.offset(i as isize) = data[i] }
        }
        self.set_addr(|r| r.set_addr(addr as u32));
        self.set_ctrla(|r| r.set_cmdex(0xa5).set_cmd(0x04));
        while !self.intflag().test_ready() {}  
        data.len()      
    }
}

