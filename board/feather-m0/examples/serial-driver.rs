#![no_std]
#![no_main]
#![feature(asm, nll)]

extern crate feather_m0 as board;
extern crate examples;

use board::prelude::*;
use board::console::SERCOM;
use board::mcu::irq::IRQ_SERCOM;
use board::mcu::sercom::SercomPeriph;

use examples::serial_driver::EchoApp;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut brd = board::init();

    let mut app: EchoApp<_, SercomPeriph> = EchoApp::new(&mut brd, SERCOM, IRQ_SERCOM).unwrap_or_abort("Unable to create app");

    brd.run(|_| app.run())

}