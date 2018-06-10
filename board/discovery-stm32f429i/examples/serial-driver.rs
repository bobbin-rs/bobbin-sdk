#![no_std]
#![no_main]

extern crate discovery_stm32f429i as board;
extern crate examples;

use board::prelude::*;
use board::mcu::usart::USART1;
use board::mcu::irq::IRQ_USART;
use board::mcu::usart::UsartPeriph;

use examples::serial_driver::EchoApp;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut brd = board::init();

    let mut app: EchoApp<_, UsartPeriph> = EchoApp::new(&mut brd, USART1, IRQ_USART).unwrap_or_abort("Unable to create app");

    brd.run(|_| app.run())

}