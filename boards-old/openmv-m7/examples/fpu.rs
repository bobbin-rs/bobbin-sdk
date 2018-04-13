#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate openmv_m7 as board;

use board::chip::fpu::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("FPU Test");

    println!("CPACR: {:?}", FPU.cpacr());
    unsafe {
        asm!("
            dsb
            isb
        ");
    }
    println!("CPACR: {:?}", FPU.cpacr());

    println!("FPU Initialized");
    let mut i: f32 = 0.1;
    println!("i: {}", i);
    for _ in 0..5 {
        i += 0.1;
        println!("i: {}", i);
    }
    println!("done");

    loop {}
    // let mut i: f32 = 0.1;
    // loop {
    //     println!("Tick {}", i);
    //     board::delay(1000);
    //     i += 0.1;
    // }
}
