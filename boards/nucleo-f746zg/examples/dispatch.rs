#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate nucleo_f746zg as board;
extern crate examples;

use board::mcu::systick::SYSTICK;
use board::mcu::ext::systick::SystickHz;
use board::mcu::scb::SCB;

use board::mcu::ext::dispatch::{HandleException};

use core::cell::UnsafeCell;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut sys = board::init();
    println!("Dispatch Test");

    // println!("{:?}", sys.dispatcher());

    let p = PendSVHandler::new();
    let p = if let Ok(p) = sys.dispatcher_mut().register_pendsv_handler(&p) {
        p
    } else {
        abort!("Unable to register PendSVHandler");
    };
    // // println!("{:?}", sys.dispatcher());

    let reload_value = (sys.clock().systick_hz() / 1000).as_u32() - 1;
    SYSTICK.set_reload_value(reload_value);
    SYSTICK.set_current_value(reload_value);
    SYSTICK.set_enabled(true);        

    
    let t = TickHandler::new();    
    let t = if let Ok(t) = sys.dispatcher_mut().register_systick_handler(&t) {
        t
    } else {
        abort!("Unable to register TickHandler 1");
    };
    // // println!("{:?}", sys.dispatcher());

    board::delay(100);

    let t2 = TickHandler::new();    
    let t2 = if let Ok(t2) = sys.dispatcher_mut().register_systick_handler(&t2) {
        t2
    } else {
        abort!("Unable to register TickHandler 2");
    };

    // // println!("{:?}", sys.dispatcher());

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
    unsafe fn handle_exception(&self, _: u8) {        
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
    unsafe fn handle_exception(&self, _: u8) {
        *self.count.get() += 1;
    }
}