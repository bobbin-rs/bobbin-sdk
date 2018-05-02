#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_l432kc as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init().run(|sys| {
        println!("Running Console");
        loop {
            sys.console().write(b"Tick...\r\n");
            sys.tick().delay(500);
        }
    })
}
