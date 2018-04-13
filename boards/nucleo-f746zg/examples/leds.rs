#![no_std]
#![no_main]

extern crate nucleo_f746zg as board;
extern crate examples;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let _ = board::init();    
    
    let b = board::board();
    let leds = [b.led0(), b.led1(), b.led2()];
    let app = examples::leds::BlinkLeds::new(&leds, b, 500);
    app.run()    
}