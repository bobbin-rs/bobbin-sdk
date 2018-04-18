#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f746zg as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut sys = board::init();

    sys.run(|_| {
        println!("Running MsTick Timeout");
        let mt = ::board::ms_tick::MS_TICK;

        println!("Expect OK after 500ms");
        let now = mt.counter();
        if let Ok(_) = mt.wait_until(1000, || mt.ticks_since(now) > 500) {
            println!("OK");
        } else {    
            println!("Timeout");
        }

        println!("Expect Timeout after 1000ms");
        let now = mt.counter();
        if let Ok(_) = mt.wait_until(1000, || mt.ticks_since(now) > 5000) {
            println!("OK");
        } else {    
            println!("Timeout");
        }    
        loop {}
    })
}
