#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f746zg as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();

    println!("Testing Exception Handler");
    board::delay(500);

    let addr = 0x1000_0000 as *const u32;
    let _ = unsafe { ::core::ptr::read_volatile(addr) };
    println!("Done");
    loop {}
}