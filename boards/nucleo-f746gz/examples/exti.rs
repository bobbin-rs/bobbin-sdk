#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate nucleo_f746zg as board;

use board::btn::BTN0;
use board::hal::gpio::*;
use board::hal::syscfg::*;
use board::hal::exti::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    
    println!("Test EXTI");

    //let b = BTN0; // PC13
    let line = EXTI_LINE13;
    SYSCFG.rcc_set_enabled(true);

    SYSCFG.set_exti(line.index(), Source::GpioC);
    
    line.set_interrupt_mask(true);
    line.set_rising_trigger(true);
    line.set_falling_trigger(true);
    line.clr_pending();   

    let h = Btn0Handler {};
    let _h = line.register_exti_handler(&h);
    let irq = line.irq_exti();
    irq.set_enabled(true);
    
    // NOTE: Doesn't seem to work if the following println is removed.
    println!("Entering Loop");

    loop {
        unsafe { asm!("wfi") }     
    }
}

pub struct Btn0Handler {}
impl HandleExti for Btn0Handler {
    fn handle_exti(&self) {
        let line = EXTI_LINE13;
        line.clr_pending();
        if !BTN0.input() {
            println!("up")
        } else {
            println!("down")
        }
    }
}
