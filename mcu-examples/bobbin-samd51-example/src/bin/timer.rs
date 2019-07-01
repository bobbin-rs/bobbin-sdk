#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m::asm;
use bobbin_samd51_example::*;
use mcu::bobbin_hal::prelude::*;
use mcu::bobbin_mcu::prelude::*;
use mcu::bobbin_hal::timer::Delay;
use mcu::tc::*;
use mcu::ext::tc::*;


#[entry]
fn main() -> ! {
    init();

    // enable gate
    TC0.gate_enable();

    let mut i = 0u32;
    loop {
        LED0.toggle_output();
        println!("Hello, World {}", i);
        i = i.wrapping_add(1);
        // TC0.delay(1000);
        // for _ in 0..2_500_000 {
        //     asm::nop();
        // }
    }
}
