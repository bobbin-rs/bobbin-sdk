#![no_std]
#![no_main]

extern crate nucleo_f031k6 as board;

use core::fmt::Write;


#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let mut u = board::usart::usart2_enabled();

    let mut i = 0;
    loop {
        write!(u, "Hello, World: {}\r\n", i).unwrap();
        i += 1;
        board::delay(1000);
    }
}

