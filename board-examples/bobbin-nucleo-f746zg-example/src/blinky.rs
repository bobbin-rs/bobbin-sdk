#![no_std]

extern crate nucleo_f746zg as board;

use board::prelude::*;

fn main() {
    let mut sys = board::init();
    sys.run(|sys| loop {
        sys.led0().toggle();
        sys.tick().delay(500);
    })
}
