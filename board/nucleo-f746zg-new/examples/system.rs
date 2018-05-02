#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f746zg as board;

use board::prelude::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut sys = board::sys_init();
    sys.run(|sys| {

        loop {}
    })
    
}
