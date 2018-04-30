#![no_std]

#[macro_use]
extern crate nucleo_f746zg as board;

use board::prelude::*;

fn main() {
    let mut sys = board::init();
    sys.run(|sys| loop {
        println!("Hello, World");
        sys.tick().delay(500);
    })
}
