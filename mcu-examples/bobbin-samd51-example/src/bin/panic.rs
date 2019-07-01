#![no_std]
#![no_main]

use cortex_m_rt::entry;
use bobbin_samd51_example::*;

#[entry]
fn main() -> ! {
    init();
    println!("preparing to panic...");
    panic!("panic!");
}

