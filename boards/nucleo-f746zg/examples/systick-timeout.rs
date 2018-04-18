#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f746zg as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let sys = board::init();

    println!("Running Systick Timeout");
    let st = ::board::systick::SYSTICK;
    let now = st.counter();
    if let Ok(_) = st.wait_until(1000, || st.ticks_since(now) > 5000) {
        println!("OK");
    } else {    
        println!("Timeout");
    }
    loop {}
}
