#![no_std]
#![no_main]
#![feature(nll)]

#[macro_use]
extern crate nucleo_f746zg as board;

use board::prelude::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut brd = board::init();
    let led = brd.led0();

    println!("Running System");
    // println!("MCU: {:?}", sys.mcu());
    // println!("Memory: {:?}", sys.memory());
    // println!("Heap: {:?}", sys.heap());
    // println!("Dispatcher: {:?}", sys.dispatcher());
    // let h = sys.heap_mut();
    // let _ = h.new(0u8);
    
    let sys = brd.sys();
    sys.run(|sys| loop {
        led.toggle();
        println!("Tick...");        
        sys.tick().delay(1000);
    })
}
