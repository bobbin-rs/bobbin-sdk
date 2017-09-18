#![no_std]
#![no_main]
#![feature(asm)]

extern crate amaph1kk_kbr as board;

use board::hal::gpio::*;


// LED0 = GPIO17 / Pin F5 / Pad 43;

#[no_mangle]
pub extern "C" fn main() -> ! {
    GPIO.with_padkey(|r| r.set_padkey(0x73));

    GPIO.with_padrege(|r| r.set_pad17fncsel(0x3));
    GPIO.with_cfgc(|r| r.set_gpio17outcfg(0x1));

    pub const DELAY: usize = 4_000_000;

    loop {
        // Set GPIO17
        GPIO.set_wtsa(|r| r.set_wtsa(1 << 17));
        // Delay approx 1/2 second
        unsafe {
            for _ in 0..DELAY { asm!("nop") }
        }
        // Clr GPIO17
        GPIO.set_wtca(|r| r.set_wtca(1 << 17));
        // ptr::write_volatile(WTCA, 1 << 17);
        // Delay approx 1/2 second
        unsafe {
            for _ in 0..DELAY { asm!("nop") }
        }
    }    
}
