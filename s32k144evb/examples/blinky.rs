#![no_std]
#![no_main]
#![feature(asm)]
extern crate s32k144evb as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let sw2 = board::sw::sw2();
    let led0 = board::led::led_blue();
    let led1 = board::led::led_red();
    led0.set(true);
    led1.set(false);
    loop {
        led0.toggle();
        led1.toggle();
        if sw2.get() {
            for _ in 0..2_000_000 {
                unsafe { asm!("nop") }
            }
        } else {
            for _ in 0..10_000_000 {
                unsafe { asm!("nop") }
            }
        }
    }
}
