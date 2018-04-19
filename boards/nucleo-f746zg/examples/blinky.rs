#![no_std]
#![no_main]
#![feature(asm)]

extern crate nucleo_f746zg as board;
extern crate examples;

use board::mcu::ext::tick::GetTick;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut brd = board::init();

    let led = brd.led0();
    let sys = brd.sys();
    
    sys.run(|sys| {
        let app = examples::led::BlinkLed::new(led, sys.mcu().tick(), 500);
        app.run()
    })

    

    // sys.run(|_| app.run())
}