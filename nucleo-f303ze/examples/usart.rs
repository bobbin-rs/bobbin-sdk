#![no_std]
#![no_main]

extern crate nucleo_f303ze as board;

use core::fmt::Write;

// USART2
// TX = PA2(AF7)
// RX = PA3(AF7)

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let mut u = board::usart::usart2(board::pin::pa2(), board::pin::pa3());
    
    let mut i = 0;
    loop {
        write!(u, "Hello, World: {}\r\n", i).unwrap();
        i += 1;
        board::delay(1000);
    }
}

