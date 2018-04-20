#![no_std]
#![no_main]
#![feature(asm, nll)]

extern crate nucleo_l432kc as board;
extern crate examples;

use board::prelude::*;
use board::console::USART;
use board::mcu::irq::IRQ_USART;
use board::mcu::usart::UsartPeriph;

use examples::serial_driver::EchoApp;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut brd = board::init();

    let mut app: EchoApp<_, UsartPeriph> = EchoApp::new(&mut brd, USART, IRQ_USART).unwrap_or_abort("Unable to create app");

    brd.run(|_| app.run())

}