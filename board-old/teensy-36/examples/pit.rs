#![no_std]
#![no_main]

#[macro_use]
extern crate teensy_36 as board;

use board::hal::{sim, pit};

// Assume PIT bus clock is 60Mhz

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    
    sim::set_pit_enabled(true);
    pit::set_enabled(true);
    let t0 = pit::timer(0);

    t0.set_load_value(60_000_000);
    t0.clr_interrupt_flag();
    t0.set_enabled(true);
    let mut i = 0u32;
    loop {
        println!("Tick {}", i);
        while t0.interrupt_flag() == false {}
        t0.clr_interrupt_flag();
        i = i.wrapping_add(1);
    }
}
