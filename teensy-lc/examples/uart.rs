#![no_std]
#![no_main]
#![feature(asm)]

extern crate teensy_lc as board;

use core::fmt::Write;

// UART0 = PA1 and PA2 / AF2

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let mut u = board::uart::uart0(board::pin::pa1(), board::pin::pa2());
    let led0 = board::led::led0();   
    let mut i = 0u32;
    loop {
        led0.toggle();
        //u.write(b"Hello, World\r\n");
        write!(u, "Hello, World {}\r\n", i).unwrap();
        i = i.wrapping_add(1);
        board::delay(1000);
        // for _ in 0..5_000_000 {
        //     unsafe { asm!("nop")}
        // }
    }
}

