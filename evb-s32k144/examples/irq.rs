#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate evb_s32k144 as board;

use board::hal::nvic;
use board::hal::port::{PORTC, GpioPin, PinExt, RegisterPortHandler, HandlePort, IrqPort};
use board::hal::gpio::{DigitalInput, DigitalOutput};
use board::hal::rtc::{RTC, PccEnabled};

use board::btn::{BTN0, BTN1};
use board::led::{LED0, LED1};

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();

    let p0 = PORTC;

    let b0 = BTN0;
    let b1 = BTN1;

    let l0 = LED0;
    let l1 = LED1;


    println!("IRQ Test");

    let n = nvic::NVIC;

    b0.set_irqc(board::hal::port::InterruptConfig::IrqEitherEdge);
    b1.set_irqc(board::hal::port::InterruptConfig::IrqEitherEdge);

    l0.set_output(true);
    l1.set_output(true);    
    let h = BtnHandler {};
    let _guard = p0.register_port_handler(&h);
    
    println!("SYSTICK_CSR: {:?}", board::chip::systick::SYSTICK.csr());

    p0.irq_port().set_enabled(true);
    while !p0.irq_port().is_enabled() {}
    // board::delay(10);    
    // board::chip::scb::SCB.with_scr(|r| r.set_sevonpend(true));
    
    let mut count = 0;    
    // Disable Interrupt Mask
    // unsafe { asm!("CPSID i") }

    println!("starting loop");
    loop {        
        if b0.isf() {
            b0.clr_isf();            
            println!("B0 {}", count);
            p0.irq_port().set_pending(false);
        }
        if b1.isf() {
            b1.clr_isf();
            println!("B1 {}", count);
            p0.irq_port().set_pending(false);
        }        
        l0.set_output(b0.gpio_pin().input());
        l1.set_output(b1.gpio_pin().input());              
        unsafe { asm!("wfi") }
    }
}

pub struct BtnHandler {}

impl HandlePort for BtnHandler {
    fn handle_port(&self) {
        let b0 = BTN0;
        let b1 = BTN1;
        if b0.isf() {
            b0.clr_isf();
            println!("B0");
        }
        if b1.isf() {
            b1.clr_isf();
            println!("B1");
        }        
    }
}
