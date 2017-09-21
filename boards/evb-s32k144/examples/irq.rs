#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate evb_s32k144 as board;

use board::hal::nvic;
use board::common::{IrqNum, WrapHandler, HandleIrq};
use board::hal::port::*;
use board::hal::gpio::*;

use board::btn::*;
use board::led::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();

    let p0 = PORTC;

    let b0 = BTN0.port_pin();
    let b1 = BTN1.port_pin();

    let l0 = LED0;
    let l1 = LED1;    

    println!("IRQ Test");

    println!("PORTC IRQ: {:?}", p0.irq_port());

    b0.set_irqc(board::hal::port::InterruptConfig::IrqEitherEdge);
    b1.set_irqc(board::hal::port::InterruptConfig::IrqEitherEdge);

    l0.set_output(true);
    l1.set_output(true);

    let bh = BtnHandler {
        b0: BTN0.port_pin().into(),
        b1: BTN1.port_pin().into(),
    };

    p0.irq_port().wrap_handler(&bh);
    
    nvic::set_enabled(p0.irq_port().irq_num() as usize, true);

    println!("starting loop");
    loop {        
        l0.set_output(!b0.gpio_pin().input());
        l1.set_output(!b1.gpio_pin().input());
    }
}

pub struct BtnHandler {
    b0: PortPin,
    b1: PortPin,
}

impl HandleIrq for BtnHandler {
    fn handle_irq(&self) {
        if self.b0.isf() {
            self.b0.clr_isf();
            println!("B0");
        }
        if self.b1.isf() {
            self.b1.clr_isf();
            println!("B1");
        }        
    }
}
