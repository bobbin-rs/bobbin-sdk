#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f746zg as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let sys = board::init();

    println!("Running Console");
    loop {
        if let Some(console) = sys.console() {
            console.write(b"Tick...\r\n");
        }
        board::delay(500);
    }
}
