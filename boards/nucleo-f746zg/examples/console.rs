#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f746zg as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let _sys = board::init();

    println!("Running Console");
    loop {
        // sys.console().write(b"Tick...\r\n");
        println!("Tick...");
        board::delay(500);
    }
}
