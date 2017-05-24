#![no_std]
#![no_main]

extern crate ek_tm4c1294xl as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let led0 = board::led::led0();
    let led1 = board::led::led1();
    let btn0 = board::btn::btn0();
    led0.toggle();
    loop {
        led0.toggle();
        led1.toggle();
        if btn0.get() {
            board::delay(500);
        } else {
            board::delay(100);
        }
    }
}
