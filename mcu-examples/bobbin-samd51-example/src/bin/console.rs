#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m::asm;
use bobbin_samd51_example::*;
use mcu::bobbin_hal::prelude::*;


#[entry]
fn main() -> ! {
    init();

    let mut i = 0u32;
    loop {
        LED0.toggle_output();
        println!("Hello, World {}", i);
        i = i.wrapping_add(1);
        for _ in 0..2_500_000 {
            asm::nop();
        }
    }
}
