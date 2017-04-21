#![no_std]
#![no_main]

extern crate discovery_stm32f4 as board;

use core::fmt::Write;

// USART1
// TX = PB6(AF7)
// RX = PB7(AF7)

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let mut u = board::usart::usart3(board::pin::pb6(), board::pin::pb7());
    
    let mut i = 0;
    loop {
        write!(u, "Hello, World: {}\r\n", i).unwrap();
        i += 1;
        board::delay(1000);
    }
}

