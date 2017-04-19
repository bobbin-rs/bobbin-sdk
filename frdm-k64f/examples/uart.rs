#![no_std]
#![no_main]

extern crate frdm_k64f as board;

use core::fmt::Write;

// UART0 = PB16 and PB17 / AF3

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let mut u = board::uart::uart0(board::pin::pb16(), board::pin::pb17());
    let led0 = board::led::led0();
    let mut i = 0;
    loop {
        led0.toggle();
        write!(u, "Hello, World: {}\r\n", i).unwrap();
        i += 1;
        board::delay(1000);
    }
}

