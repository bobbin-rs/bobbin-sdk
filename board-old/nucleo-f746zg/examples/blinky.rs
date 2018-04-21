#![no_std]
#![feature(asm)]

extern crate nucleo_f746zg as board;
use board::led::*;
use board::btn::*;

fn main() {
    board::init();
    loop {
        LED0.toggle_output();
        if BTN0.input() {
            board::delay(500);
        } else {
            board::delay(100);
        }
    }
}
