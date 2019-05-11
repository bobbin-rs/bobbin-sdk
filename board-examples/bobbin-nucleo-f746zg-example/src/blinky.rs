#![no_std]
#![no_main]

extern crate nucleo_f746zg as board;
extern crate cortex_m_rt;

use board::prelude::*;
use cortex_m_rt::entry;

#[entry]
unsafe fn main() -> ! {
    let mut sys = board::init();
    sys.run(|sys| loop {
        sys.led0().toggle();
        sys.tick().delay(500);
    })
}
