#![no_std]
#![no_main]
#![feature(asm)]

extern crate arduino_zero as board;
extern crate examples;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();    
    let brd = board::board();
    let app = examples::btn::BtnExample::new(brd.btn0(), brd.led0(), brd, 500, 100);
    app.run()
}
