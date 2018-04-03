#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate nucleo_f746zg as board;
extern crate examples;

use board::mcu::systick;

use board::mcu::bobbin_common::dispatch::*;
static mut HANDLER_SLOTS: [Option<IrqHandler>; 2] = [None; 2];

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Dispatch Test");

    unsafe {
        Dispatcher::init(&mut HANDLER_SLOTS)
    }
    let mut t = TickHandler::new();
    let _h = Dispatcher::register_handler(IrqHandler::new(15, &mut t));

    let reload_value = (216_000_000 / 8000) - 1;
    println!("Setting reload_value to {}", reload_value);    
    systick::SYSTICK.set_reload_value(reload_value);
    systick::SYSTICK.set_current_value(reload_value);
    systick::SYSTICK.set_enabled(true);
    systick::SYSTICK.set_tick_interrupt(true);
    
    loop {
        println!("tick: {} {}", t.count, t.irq);
        board::delay(1000);
    }
}

pub struct TickHandler {
    count: u32,
    irq: u8,
}

impl TickHandler {
    pub fn new() -> Self {
        TickHandler { count: 0, irq: 0 }
    }
}

impl HandleIrq for TickHandler {
    unsafe fn handle_irq(&mut self, irq: u8) -> IrqResult {
        self.irq = irq;
        self.count += 1;
        IrqResult::Continue
    }
}