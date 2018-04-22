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

// pub trait FlashLockUnlock {
//     fn flash_locked(&self) -> bool;
//     fn flash_unlock(&self);
//     fn flash_lock(&self);
//     fn unlocked<T, F: FnOnce(&Self)->T>(&self, f: F) -> T {
//         self.flash_unlock();
//         let ret = f(self);
//         self.flash_lock();
//         ret
//     }
// }

// pub trait FlashBusy {
//     fn flash_busy(&self) -> bool;
// }

// pub trait FlashErase {
//     fn flash_erase(&self, addr: *const u32);
// }

// pub trait FlashWrite<T> {
//     fn flash_write(&self, addr: *mut T, data: &[T]) -> usize;
// }

// pub const KEY1: u32 = 0x45670123;
// pub const KEY2: u32 = 0xCDEF89AB;

// impl FlashLockUnlock for FlashPeriph {
//     fn flash_locked(&self) -> bool {
//         self.cr().test_lock()
//     }

//     fn flash_lock(&self) {
//         self.with_cr(|r| r.set_lock(1));
//     }

//     fn flash_unlock(&self) {
//         self.set_keyr(|r| r.set_key(KEY1));
//         self.set_keyr(|r| r.set_key(KEY2));
//     }
// }

// impl FlashBusy for FlashPeriph {
//     fn flash_busy(&self) -> bool {
//         self.sr().test_bsy()
//     }
// }

// impl FlashErase for FlashPeriph {    
//     fn flash_erase(&self, addr: *const u32) {
//         while self.flash_busy() {}
//         let snb = match addr as u32 {
//             0x0800_0000 => 0,
//             0x0800_8000 => 1,
//             0x0801_0000 => 2,
//             0x0801_8000 => 3,
//             0x0802_0000 => 4,
//             0x0804_0000 => 5,
//             0x0808_0000 => 6,
//             0x080c_0000 => 7,
//             _ => panic!("Invalid flash sector address"),
//         };        
//         self.with_cr(|r| r.set_snb(snb).set_ser(1).set_strt(1));
//         while self.flash_busy() {}
//         self.with_cr(|r| r.set_ser(0));
//     }
// }

// impl FlashWrite<u8> for FlashPeriph {
//     fn flash_write(&self, addr: *mut u8, data: &[u8]) -> usize {
//         while self.flash_busy() {}
//         self.with_cr(|r| r.set_pg(1).set_psize(0b00));
//         for i in 0..data.len() {
//             unsafe {
//                 *addr.offset(i as isize) = data[i];
//                 asm!("dsb");
//             }
//             while self.flash_busy() {}
//         }
//         self.with_cr(|r| r.set_pg(0));        
//         data.len()
//     }
// }


// impl FlashWrite<u16> for FlashPeriph {
//     fn flash_write(&self, addr: *mut u16, data: &[u16]) -> usize {
//         while self.flash_busy() {}
//         self.with_cr(|r| r.set_pg(1).set_psize(0b01));
//         for i in 0..data.len() {
//             unsafe {
//                 *addr.offset(i as isize) = data[i];
//                 asm!("dsb");
//             }
//             while self.flash_busy() {}
//         }
//         self.with_cr(|r| r.set_pg(0));        
//         data.len()
//     }
// }


// impl FlashWrite<u32> for FlashPeriph {
//     fn flash_write(&self, addr: *mut u32, data: &[u32]) -> usize {
//         while self.flash_busy() {}
//         self.with_cr(|r| r.set_pg(1).set_psize(0b10));
//         for i in 0..data.len() {
//             unsafe {
//                 *addr.offset(i as isize) = data[i];
//                 asm!("dsb");
//             }
//             while self.flash_busy() {}
//         }
//         self.with_cr(|r| r.set_pg(0));
//         data.len()
//     }
// }

