#![no_std]
#![feature(asm)]

#[macro_use] extern crate nucleo_f746zg as board;

use board::mcu::pin::*;
use board::mcu::rcc::*;

fn main() {
    board::init();
    println!("MCO2 on PC9");
    PC9.port().gate_enable();
    PC9.mode_alt_fn(0).speed_high();
    RCC.with_cfgr(|r| r.set_mco2(0b10).set_mco2pre(0b000)); 
    loop {}
}
