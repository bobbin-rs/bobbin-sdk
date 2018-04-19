#![no_std]
#![no_main]
#![feature(asm)]

extern crate nucleo_l432kc as board;
extern crate examples;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let brd = board::board();
    let mut sys = board::init();

    sys.run(|sys| {        
        let app = examples::led::BlinkLed::new(brd.led0(), sys.tick(), 500);
        app.run()
    })
}
