#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate discovery_stm32f3 as board;
extern crate examples;

use board::mcu::systick::SYSTICK;
use board::mcu::systick_ext::SystickHz;
use board::mcu::scb::SCB;
use board::clock::*;

use board::mcu::dispatch::{HandleException, Exception};
use board::Dispatcher;

use core::cell::UnsafeCell;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Dispatch Test");

    println!("{} / {} slots allocated", Dispatcher::slots_used(), Dispatcher::slots());

    let p = PendSVHandler::new();
    let p = Dispatcher::register_pendsv_handler(&p).unwrap();
    println!("{} / {} slots allocated", Dispatcher::slots_used(), Dispatcher::slots());

    println!("Systick Hz: {}", Clk::systick_hz().as_u32());

    let reload_value = (Clk::systick_hz() / 1000).as_u32() - 1;
    println!("Systick Reload Value: {}", reload_value);

    SYSTICK.set_reload_value(reload_value);
    SYSTICK.set_current_value(reload_value);
    SYSTICK.set_enabled(true);        

    println!("Systick Enabled");

    let t = TickHandler::new();    
    let t = Dispatcher::register_systick_handler(&t).unwrap();
    println!("{} / {} slots allocated", Dispatcher::slots_used(), Dispatcher::slots());

    // board::delay(100);

    // let t2 = TickHandler::new();    
    // let t2 = Dispatcher::register_systick_handler(&t2).unwrap();

    // println!("{} / {} slots allocated", Dispatcher::slots_used(), Dispatcher::slots());

    println!("Starting Loop");

    loop {
        // println!("tick: {} {} {}", unsafe { *t.count.get()}, unsafe { *t2.count.get()} , unsafe { *p.count.get()} );
        println!("tick");
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