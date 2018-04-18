#![no_std]
#![no_main]

#[macro_use]
extern crate arduino_zero as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut sys = board::init();
    sys.run(|sys| {
        println!("Running Console");
        loop {
            if let Some(console) = sys.console() {
                console.write(b"Tick...\r\n");
            }
            sys.tick().delay(500);
        }
    })
}