#![no_std]
#![no_main]

extern crate nucleo_f031k6 as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let led0 = board::led::led0();
    let btn0 = board::btn::btn0();
    loop {
        led0.toggle();
        if btn0.get() {
            board::delay(1_000_000);
        } else {
            board::delay(100_000);
        }        
    }
}
