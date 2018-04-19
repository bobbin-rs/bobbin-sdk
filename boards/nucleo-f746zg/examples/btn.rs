#![no_std]
#![no_main]
#![feature(asm)]

extern crate nucleo_f746zg as board;
extern crate examples;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut brd = board::board();
    let btn = brd.btn0();
    let led = brd.led0();
    let sys = brd.sys();
   
    let app = examples::btn::BtnExample::new(btn, led, sys.tick(), 500, 100);
    sys.run(|_| app.run())
}
