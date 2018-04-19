#![no_std]
#![no_main]
#![feature(asm)]

extern crate nucleo_f746zg as board;
extern crate examples;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut brd = board::init();

    let led = brd.led0();
    let sys = brd.sys();
    let app = examples::led::BlinkLed::new(led, sys.tick(), 500);

    sys.run(|_| app.run())
}