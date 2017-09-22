#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]

// #[macro_use]
extern crate silabs_slstk3401a as board;
use board::hal::gpio::*;

use core::ptr;

// LED0 = PF4;

pub const PIN: usize = 4;

pub const CMU_HFBUSCLKEN0: *mut u32 = 0x400e_40b0 as *mut u32;
pub const CMU_HFBUSCLKEN0_GPIO: u32 = 1 << 3;

// GPIO Base: 0x4000A000
// GPIO_PF Base: 0x4000a0dc

pub const GPIO_PF_CTRL: *mut u32 = 0x4000_a0f0 as *mut u32;
pub const GPIO_PF_MODEL: *mut u32 = 0x4000_a0f4 as *mut u32;
pub const GPIO_PF_MODEH: *mut u32 = 0x4000_a0f8 as *mut u32;
pub const GPIO_PF_DOUTTGL: *mut u32 = 0x4000_a108 as *mut u32;

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        // Enable GPIO
        ptr::write_volatile(CMU_HFBUSCLKEN0, ptr::read_volatile(CMU_HFBUSCLKEN0) | CMU_HFBUSCLKEN0_GPIO);
        // Set PF4 Mode = Output
        ptr::write_volatile(GPIO_PF_MODEL, ptr::read_volatile(GPIO_PF_MODEL) | 0b0100 << (PIN * 4));
        loop {
            // Toggle PF4
            // ptr::write_volatile(GPIO_PF_DOUTTGL, 1 << PIN);
            GPIOF.set_douttgl(|r| r.set_douttgl(PIN, 1));
            // Delay approx 1/2 second
            for _ in 0..5_000_000 { asm!("nop") }
        }
    }
}
