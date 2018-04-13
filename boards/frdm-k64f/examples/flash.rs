#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate frdm_k64f as board;

extern "C" {
    static mut _stext: u32;
}

use core::slice;
use board::mcu::ftfe::*;

pub const FLASH_ADDR: *mut u32 = 0x008_0000 as *mut u32;
pub const FLASH_LEN: usize = 0x100;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let _ = board::init();
    use board::mcu::sim::*;
    board::delay(1000);
    println!("Flash Test");
    println!("Disable MPU");

    println!("SIM_FCFG1: {:?}", SIM.fcfg1());
    println!("FCNFG: {:?}", FTFE.fcnfg());
    println!("FSEC:  {:?}", FTFE.fsec());
    println!("FOPT:  {:?}", FTFE.fopt());

    for i in 0..4 {
        println!("FPROT{}: {:?}", i, FTFE.fprot(i));
    }
    println!("FEPROT: {:?}", FTFE.feprot());
    println!("FDPROT: {:?}", FTFE.fdprot());

    dump(FLASH_ADDR as *const u8, FLASH_LEN);    
    {
        println!("flash erase {:p}", FLASH_ADDR);
        FTFE.unlocked(|f| f.flash_erase(FLASH_ADDR));
        println!("flash erase done")
    }
    // dump(FLASH_ADDR as *const u8, FLASH_LEN);    
    {
        println!("Flash write");
        let mut buf = [0u32; 0x100 / 4];
        for i in 0..buf.len() {
            buf[i] = i as u32;
        }
        FTFE.unlocked(|f| {
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

pub trait FtfeWriteCommand {
    fn write_command(&self, buf: &[u8]);
}

impl FtfeWriteCommand for FtfePeriph {
    fn write_command(&self, buf: &[u8]) {
        self.set_fstat(|_| Fstat(0x70));
        while self.flash_busy() {}
        // println!("fstat: {:?}", self.fstat());
        for i in 0..buf.len() {
            self.set_fccob(i, |_| Fccob(buf[i]));
            // print!("{:02x} ", self.fccob(i).0);
        }
        // println!("");
        self.set_fstat(|_| Fstat(0x80));
        // println!("fstat: {:?}", self.fstat());
    }
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


impl FlashLockUnlock for FtfePeriph {
    fn flash_locked(&self) -> bool {
        false
    }

    fn flash_lock(&self) {
    }

    fn flash_unlock(&self) {
    }
}

impl FlashBusy for FtfePeriph {
    fn flash_busy(&self) -> bool {
        !self.fstat().test_ccif()
    }
}

impl FlashErase for FtfePeriph {    
    fn flash_erase(&self, addr: *const u32) {
        let addr = addr as u32;
        self.write_command(&[
            (addr >> 0) as u8,
            (addr >> 8) as u8,
            (addr >> 16) as u8,
            0x08,
        ]);
    }
}


impl FlashWrite<u32> for FtfePeriph {
    fn flash_write(&self, addr: *mut u32, data: &[u32]) -> usize {
        let addr = addr as u32;
        let mut i = 0;
        while i < data.len() {
            let daddr = addr + (i as u32 * 4);
            let data_1 = data[i];
            let data_2 = data[i + 1];
            self.write_command(&[
                (daddr >> 0) as u8,
                (daddr >> 8) as u8,
                (daddr >> 16) as u8,
                0x07,
                (data_1 >> 0) as u8,
                (data_1 >> 8) as u8,
                (data_1 >> 16) as u8,
                (data_1 >> 24) as u8,
                (data_2 >> 0) as u8,                
                (data_2 >> 8) as u8,
                (data_2 >> 16) as u8,
                (data_2 >> 24) as u8,
            ]);
            i += 2;
        }
        // println!("FSTAT: {:?}", self.fstat());        
        data.len()
    }
}

