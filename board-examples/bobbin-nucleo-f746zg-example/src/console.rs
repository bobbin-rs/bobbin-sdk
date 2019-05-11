#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f746zg as board;
extern crate cortex_m_rt;

use cortex_m_rt::entry;
use board::prelude::*;

#[entry]
unsafe fn main() -> ! {
    let mut sys = board::init();
    sys.run(|sys| loop {
        println!("Hello, World");
        sys.tick().delay(500);
    })
}
