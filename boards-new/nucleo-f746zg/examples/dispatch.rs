#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate nucleo_f746zg as board;
extern crate examples;

use board::mcu::systick::SYSTICK;
use board::mcu::scb::SCB;

use board::ext::{Dispatcher, ExceptionHandler, HandleException, Exception};

use core::cell::UnsafeCell;

static mut HANDLER_SLOTS: [Option<ExceptionHandler>; 4] = [None; 4];

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Dispatch Test");

    unsafe {
        Dispatcher::init(&mut HANDLER_SLOTS)
    }

    let p = PendSVHandler::new();
    let p = Dispatcher::register_pendsv_handler(&p).unwrap();

    let reload_value = (216_000_000 / 8000) - 1;
    SYSTICK.set_reload_value(reload_value);
    SYSTICK.set_current_value(reload_value);
    SYSTICK.set_enabled(true);        

    
    let t = TickHandler::new();    
    let t = Dispatcher::register_systick_handler(&t).unwrap();

    board::delay(100);

    let t2 = TickHandler::new();    
    let t2 = Dispatcher::register_systick_handler(&t2).unwrap();


    loop {
        println!("tick: {} {} {}", unsafe { *t.count.get()}, unsafe { *t2.count.get()} , unsafe { *p.count.get()} );
        board::delay(1000);
    }
}



pub struct TickHandler {
    count: UnsafeCell<u32>,
}

impl TickHandler {
    pub fn new() -> Self {
        Self { count: UnsafeCell::new(0) }
    }
}

impl HandleException for TickHandler {
    unsafe fn handle_exception(&self, _exc: Exception) {        
        *self.count.get() += 1;
        if *self.count.get() % 1000 == 0 {
            SCB.with_icsr(|r| r.set_pendsvset(1));
        }
    }
}

pub struct PendSVHandler {
    count: UnsafeCell<u32>,
}

impl PendSVHandler {
    pub fn new() -> Self {
        Self { count: UnsafeCell::new(0) }
    }
}

impl HandleException for PendSVHandler {
    unsafe fn handle_exception(&self, _exc: Exception) {
        *self.count.get() += 1;
    }
}