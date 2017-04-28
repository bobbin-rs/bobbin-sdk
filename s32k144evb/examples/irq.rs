#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate s32k144evb as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let sw2 = board::sw::sw2();
    let sw3 = board::sw::sw3();
    println!("IRQ Test");
    sw2.port().set_handler(Some(handle_port));
    sw2.set_irqc(board::hal::port::InterruptConfig::IrqEitherEdge);
    sw3.set_irqc(board::hal::port::InterruptConfig::IrqEitherEdge);

    let led0 = board::led::led_blue();
    let led1 = board::led::led_red();
    led0.set(true);
    led1.set(true);
    loop {
        led0.set(!sw2.get());
        led1.set(!sw3.get());
        unsafe { asm!("wfi") }
    }
}

unsafe extern "C" fn handle_port() {
    let mut port = board::chip::port::PORTC;    
    println!("irq: {:?}", port.isfr());
    port.set_isfr(board::chip::port::Isfr(0xffffffff));
    
}