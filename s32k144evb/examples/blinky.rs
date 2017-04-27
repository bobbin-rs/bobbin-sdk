#![no_std]
#![no_main]
#![feature(asm)]
extern crate s32k144evb as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    // unsafe {
    //     // Enable PCC for PORTD
    //     ptr::write_volatile(0x4006_5130 as *mut u32, 0xc0000000);
    //     // Configure PORTD @ 0x4004c000 for GPIO
    //     ptr::write_volatile(0x4004_c000 as *mut u32, 0x0000_0100);
    //     // Configure GPIOD @ 0x400f0c0 pin 0 for output
    //     ptr::write_volatile(0x400f_f0d4 as *mut u32, 0x0000_0001);
    //     loop {
    //         ptr::write_volatile(0x400f_f0cc as *mut u32, 0x0000_0001);
    //         for _ in 0..1_000_000 {
    //             asm!("nop");
    //         }
    //         // ptr::write_volatile(0x400f_f0c8 as *mut u32, 0x0000_0001);
    //         // for _ in 0..10_000 {
    //         //     asm!("nop");
    //         // }
    //     }
    // }


    board::init();
    let led0 = board::led::led0();
    loop {
        led0.set(true);
        for _ in 0..4_000_000 {
            unsafe { asm!("nop") }
        }
        led0.set(false);
        for _ in 0..4_000_000 {
            unsafe { asm!("nop") }
        }
    }
}
