#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate feather_m0 as board;
extern crate examples;

use board::mcu::systick::SYSTICK;
use board::mcu::systick_ext::SystickHz;
use board::mcu::scb::SCB;

use board::mcu::dispatch::{HandleException, Exception};
use core::cell::UnsafeCell;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut sys = board::init();
    println!("Dispatch Test");

    // println!("{:?}", sys.dispatcher());

    let p = PendSVHandler::new();
    let p = sys.dispatcher_mut().register_pendsv_handler(&p).unwrap();
    // println!("{:?}", sys.dispatcher());

    let reload_value = (sys.clock().systick_hz() / 1000).as_u32() - 1;
    SYSTICK.set_reload_value(reload_value);
    SYSTICK.set_current_value(reload_value);
    SYSTICK.set_enabled(true);        

    
    let t = TickHandler::new();    
    let t = sys.dispatcher_mut().register_systick_handler(&t).unwrap();
    // println!("{:?}", sys.dispatcher());

    board::delay(100);

    let t2 = TickHandler::new();    
    let t2 = sys.dispatcher_mut().register_systick_handler(&t2).unwrap();

    // println!("{:?}", sys.dispatcher());

    sys.run(|sys| loop {
        // println!("tick: {} {} {}", t.count(), t2.count(), p.count());
        if let Some(c) = sys.console() {
            c.write(b"Tick: ");
            c.write_u32(t.count(), 10);
            c.write(b" ");
            c.write_u32(t2.count(), 10);
            c.write(b" ");
            c.write_u32(p.count(), 10);
            c.write(b"\n");
        }
        board::delay(1000);        
    })

}






pub struct TickHandler {
    count: UnsafeCell<u32>,
}

impl TickHandler {
    pub fn new() -> Self {
        Self { count: UnsafeCell::new(0) }
    }

    pub fn count(&self) -> u32 {
        unsafe { * self.count.get() }
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

    pub fn count(&self) -> u32 {
        unsafe { * self.count.get() }
    }
    
}

impl HandleException for PendSVHandler {
    unsafe fn handle_exception(&self, _exc: Exception) {
        *self.count.get() += 1;
    }
}