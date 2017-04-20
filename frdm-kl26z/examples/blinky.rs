#![no_std]
#![no_main]

extern crate frdm_kl26f as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let led0 = board::led::led0();
    let sw2 = board::sw::sw2();
    loop {
        led0.toggle();
        board::delay(100);
        // if sw2.get() {
        //     board::delay(500);
        // } else {
        //     board::delay(250);            
        // }        
    }
}
