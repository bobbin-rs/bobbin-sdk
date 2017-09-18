#![no_std]
#![no_main]
#![feature(asm)]

extern crate amaph1kk_kbr as board;

use board::hal::gpio::*;


// LED0 = GPIO17 / Pin F5 / Pad 43;

#[no_mangle]
pub extern "C" fn main() -> ! {
    // GPIO.with_padkey(|r| r.set_padkey(0x73));

    let led = P17;

    GPIO
        .unlock();

    led
        .set_pad_gpio()
        .set_gpio_output_pushpull();

    // GPIO.with_padreg(4, |r| r.set_padfncsel(1, 0x3));
    // GPIO.with_cfg(2, |r| r.set_gpiooutcfg(1, 0x1));

    pub const DELAY: usize = 4_000_000;

    loop {
        // Set GPIO17
        led.set_output(true);
        // GPIO.set_wts(0, |r| r.set_wts(17, 1));
        // Delay approx 1/2 second
        unsafe {
            for _ in 0..DELAY { asm!("nop") }
        }
        // Clr GPIO17
        led.set_output(false);
        // GPIO.set_wtc(0, |r| r.set_wtc(17, 1));
        // ptr::write_volatile(WTCA, 1 << 17);
        // Delay approx 1/2 second
        unsafe {
            for _ in 0..DELAY { asm!("nop") }
        }
    }    
}
