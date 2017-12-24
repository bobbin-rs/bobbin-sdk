#![no_std]
#![no_main]

#[macro_use]
extern crate audio_node as board;

use board::hal::usart::*;
use board::console::USART;
use core::fmt::Write;


#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Running Serial Test");
    let irq = board::console::USART.irq_usart();
    let mut tx_buf = [0u8; 16];
    let mut rx_buf = [0u8; 16];
    let mut s = UsartDriver::new(USART, &mut tx_buf, &mut rx_buf);
    s.enable_irq(&irq);

    write!(s, "Hello, World!\r\n").unwrap();
    s.enable_rx();


    let mut buf = [0u8; 16];
    loop {
        let n = s.read(&mut buf);
        if n > 0 {
            for b in (&buf[..n]).iter() {
                if *b == b'\r' {
                    s.write(b"\r\n");
                } else {
                    s.write(&[*b]);
                }
            }
        }
    }
}
