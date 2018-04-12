#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f746zg as board;

use core::fmt::Write;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut sys = board::ext::System::init();

    println!("Running System");
    println!("MCU: {:?}", sys.mcu());
    println!("Memory: {:?}", sys.memory());
    println!("Heap: {:?}", sys.heap());
    println!("Dispatcher: {:?}", sys.dispatcher());

    sys.run(|sys| {
        loop {
            sys.with_console(|c| {
                let _ = c.write_str("Tick\n");
            });
            board::delay(500);
        }
    })
}
