#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate nucleo_f746zg as board;
extern crate examples;

use board::mcu::systick;

use board::ext::dispatch::*;
use board::ext::{Dispatcher, IrqHandler};

use core::cell::UnsafeCell;

static mut HANDLER_SLOTS: [Option<IrqHandler>; 2] = [None; 2];

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Dispatch Test");

    unsafe {
        Dispatcher::init(&mut HANDLER_SLOTS)
    }

    let reload_value = (216_000_000 / 8000) - 1;

    let t = TickHandler::new();    
    let t = Dispatcher::register_handler(15, &t).unwrap();
    t.configure(reload_value);
    t.enable();

    board::delay(100);

    let t2 = TickHandler::new();    
    let t2 = Dispatcher::register_handler(15, &t2).unwrap();


    loop {
        println!("tick: {} {}", unsafe { *t.count.get()}, unsafe { *t2.count.get()}  );
        board::delay(1000);
    }
}

use systick::SYSTICK;

pub struct TickHandler {
    count: UnsafeCell<u32>,
}

impl TickHandler {
    pub fn new() -> Self {
        Self { count: UnsafeCell::new(0) }
    }
    
    pub fn configure(&self, reload_value: u32) {
        SYSTICK.set_reload_value(reload_value);
        SYSTICK.set_current_value(reload_value);
    }

    pub fn enable(&self) {
        SYSTICK.set_tick_interrupt(true);
        SYSTICK.set_enabled(true);        
    }
}

impl HandleIrq for TickHandler {
    unsafe fn handle_irq(&self, _irq: u8) -> IrqResult {
        *self.count.get() += 1;
        IrqResult::Continue
    }
}