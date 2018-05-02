#![no_std]
#![no_main]

extern crate nucleo_f746zg as board;

use board::prelude::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut brd = board::init();
    brd.run(|brd| {
        if let Some(console) = brd.console() {
            console.write(b"0x");
            console.write_u8_hex(0xab);
            console.write(b"\n");
            console.write(b"0x");
            console.write_u16_hex(0xabcd);
            console.write(b"\n");
            console.write(b"0x");
            console.write_u32_hex(0xabcd1234);
            console.write(b"\n");
        }
        loop {}
    })
}