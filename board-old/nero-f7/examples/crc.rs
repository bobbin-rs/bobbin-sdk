#![no_std]
#![no_main]

#[macro_use]
extern crate nero_f7 as board;

use board::hal::crc::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();        
    println!("Running CRC Test");

    CRC.rcc_set_enabled(true);
    CRC.reset();

    let mut i = 0;
    loop {
        println!("CRC: {:08x}", CRC.read());
        CRC.write(i);
        i += 1;
        board::delay(1_000);
    }
}

