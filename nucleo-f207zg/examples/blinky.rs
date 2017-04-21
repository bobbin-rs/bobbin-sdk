#![no_std]
#![no_main]

extern crate nucleo_f207zg as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let led0 = board::led::led0();
    let btn0 = board::btn::btn0();
    loop {
        led0.toggle();
        if btn0.get() {            
            board::delay(500);
        } else {
            board::delay(250);            
        }        
    }
}
