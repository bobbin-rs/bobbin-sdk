#![no_std]
#![no_main]
#![feature(asm)]

extern crate frdm_k64f as board;
extern crate examples;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut sys = board::init();    
    let brd = board::board();
    let app = examples::btn::BtnExample::new(brd.btn0(), brd.led0(), sys.tick(), 500, 100);
    sys.run(|_| app.run())
}
