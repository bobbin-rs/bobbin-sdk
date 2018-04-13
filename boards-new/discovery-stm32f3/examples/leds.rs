#![no_std]
#![no_main]

extern crate discovery_stm32f3 as board;
extern crate examples;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let _ = board::init();    
    
    let b = board::board();
    let leds = [b.led0(), b.led1(), b.led2(), b.led3(), b.led4(), b.led5(), b.led6(), b.led7()];
    let app = examples::leds::BlinkLeds::new(&leds, b, 100);
    app.run()    
}