#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_l432kc as board;

use board::console;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Running Echo");
    loop {
        if let Some(c) = console::try_getc() {
            println!("{:02x}: {}", c, c as char);
        }
    }
}
