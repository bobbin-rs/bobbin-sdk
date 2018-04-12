#![no_std]
#![no_main]
#![feature(asm)]

extern crate nucleo_f746zg as board;
extern crate examples;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut sys = board::ext::System::init();
    let brd = board::board();
    let app = examples::led::BlinkLed::new(brd.led0(), brd, 500);
    sys.run(|_| app.run())
}
