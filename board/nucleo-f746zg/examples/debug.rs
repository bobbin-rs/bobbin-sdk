#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f746zg as board;

use board::prelude::*;
use board::mcu::usart::USART3;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init().run(|_| {
        println!("USART3.CR1:   {:?}", USART3.cr1());
        println!("USART3.CR2:   {:?}", USART3.cr2());
        println!("USART3.BRR:   {:?}", USART3.brr());
        loop {}
    })
}
