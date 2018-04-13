#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f746zg as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut sys = board::System::init();

    println!("Running System");
    println!("MCU: {:?}", sys.mcu());
    println!("Memory: {:?}", sys.memory());
    println!("Heap: {:?}", sys.heap());
    println!("Dispatcher: {:?}", sys.dispatcher());

    sys.run(|sys| {
        loop {
            sys.console().write(b"Tick\n");
            board::delay(500);
        }
    })
}
