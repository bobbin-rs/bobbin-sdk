#![no_std]
#![no_main]

#[macro_use]
extern crate evb_s32k144 as board;

use core::fmt::Write;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    // Use external UART
    let mut u = board::serial::serial0();
    let led0 = board::led::led_red();
    let mut i = 0;
    println!("UART Test - Output on LPUART0");
    write!(u, "UART Test on LPUART0\r\n").unwrap();
    loop {
        if let Some(c) = u.try_getc() {
            write!(u, "{}: {:?}\r\n", i, c as char).unwrap();
            led0.toggle();        
            i += 1;
        }        
    }
}

