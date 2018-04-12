#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f746zg as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut sys = board::ext::System::init();

    println!("Running System");
    println!("Memory: {:?}", sys.memory());
    println!("Heap: {:?}", sys.heap());
    println!("Dispatcher: {:?}", sys.dispatcher());

    sys.run(|_| {
        loop {
            println!("Tick...");
            board::delay(500);
        }
    })
}
