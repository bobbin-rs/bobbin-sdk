#![no_std]
#![no_main]

extern crate ek_tm4c1294xl as board;

use core::fmt::Write;

// UART0
// TX = PA0(1)
// RX = PA1(1)

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let mut u = board::uart::uart0(board::pin::pa0(), board::pin::pa1());
    
    let mut i = 0;
    loop {
        write!(u, "Hello, World: {}\r\n", i).unwrap();
        i += 1;
        board::delay(1000);
    }
}

