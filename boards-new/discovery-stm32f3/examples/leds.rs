#![no_std]
#![no_main]

extern crate discovery_stm32f3 as board;
extern crate examples;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();    
    
    let b = board::board();
    let leds = [b.led0(), b.led1(), b.led2()];
    let app = examples::leds::BlinkLeds::new(&leds, b, 500);
    app.run()    
}