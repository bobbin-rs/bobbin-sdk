#![no_std]
#![no_main]
#![feature(asm, nll)]

extern crate frdm_k64f as board;
extern crate examples;

use board::prelude::*;
use board::console::UART;
use board::mcu::irq::IRQ_UART;
use board::mcu::uart::UartPeriph;

use examples::serial_driver::EchoApp;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut brd = board::init();

    let mut app: EchoApp<_, UartPeriph> = EchoApp::new(&mut brd, UART, IRQ_UART).unwrap_or_abort("Unable to create app");

    brd.run(|_| app.run())

}
