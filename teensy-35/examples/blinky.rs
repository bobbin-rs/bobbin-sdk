#![no_std]
#![no_main]

extern crate teensy_35 as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let led0 = board::led::led0();
    let sw2 = board::sw::sw2();
    loop {
        led0.toggle();
        if sw2.get() {
            board::delay(500);
        } else {
            board::delay(250);            
        }        
    }
}
