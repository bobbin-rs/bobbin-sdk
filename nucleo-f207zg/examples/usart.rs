#![no_std]
#![no_main]

extern crate nucleo_f207zg as board;

use core::fmt::Write;

// USART2
// TX = PD8(AF7)
// RX = PD9(AF7)

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let mut u = board::usart::usart3(board::pin::pd8(), board::pin::pd9());
    
    let mut i = 0;
    loop {
        write!(u, "Hello, World: {}\r\n", i).unwrap();
        i += 1;
        board::delay(1000);
    }
}

