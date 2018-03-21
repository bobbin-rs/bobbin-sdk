#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f746zg as board;

use board::mcu::MCU;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();

    println!("MCU Test");

    println!("GPIOA_MODER: {:?}", MCU.gpioa().moder());


    loop {
        println!("Tick...");
        board::delay(500);
    }
}
