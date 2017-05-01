#![no_std]
#![no_main]

extern crate evb_s32k144 as board;

use core::fmt::Write;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let mut u = board::serial::serial1();
    let led0 = board::led::led_red();
    let mut i = 0;
    write!(u, "Running LPUART Test\r\n").unwrap();
    loop {
        if let Some(c) = u.try_getc() {
            write!(u, "{}: {:?}\r\n", i, c as char).unwrap();
            led0.toggle();        
            i += 1;
        }        
    }
}

