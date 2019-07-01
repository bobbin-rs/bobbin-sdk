#![no_std]
#![no_main]

extern crate panic_abort;
use cortex_m_rt::entry;
use cortex_m::asm;
use core::ptr;

// LED Pin D13 = a23

// const PORTA: u32 = 0x4100_8000;
const PORTA_DIRSET: u32 = 0x4100_8008;
const PORTA_OUTTGL: u32 = 0x4100_801C;

#[entry]
fn main() -> ! {

    // Enable PORT bus clock (CLK_PORT_APB)
    // Set a23 as output
    // DIRSET |= 1 << 23

    unsafe {
        ptr::write_volatile(PORTA_DIRSET as *mut u32, 1 << 23);
    }

    // 
    // Set a23 high / set a23 low

    loop {
        for _ in 0..1_000_000 {
            asm::nop();
        }
        unsafe {
            ptr::write_volatile(PORTA_OUTTGL as *mut u32, 1 << 23);
        }        
    }
}