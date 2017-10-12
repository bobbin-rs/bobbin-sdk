#![no_std]
#![no_main]

#[macro_use]
extern crate frdm_k64f as board;

use board::console::UART;
use board::hal::uart::*;
use core::fmt::Write;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();    
    println!("Running Serial Driver");

    let mut tx_buf = [0u8; 64];
    let mut rx_buf = [0u8; 64];

    let mut u = UartDriver::new(UART, &mut tx_buf, &mut rx_buf);
    u.enable_irq(&UART.irq_uart());
    let mut i = 0;
    loop {
        writeln!(u, "Hello, World {}", i).unwrap();
        i += 1;
        board::delay(1000);
    }
}

