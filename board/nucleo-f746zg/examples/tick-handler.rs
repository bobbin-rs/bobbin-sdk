#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f746zg as board;

use board::prelude::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut sys = board::init();
    
    let ticker = Ticker::new();
    let _guard = sys.tick_mut().register(&ticker);

    sys.run(|sys| {
        loop {
            println!("Tick... {}", ticker.counter());    
            sys.tick().delay(500);
        }
    })
}

use core::cell::UnsafeCell;

pub struct Ticker {
    counter: UnsafeCell<u32>,
}

impl Ticker {
    pub fn new() -> Self {
        Self { counter: UnsafeCell::new(0) }        
    }

    pub fn counter(&self) -> u32 {
        unsafe { *self.counter.get() }
    }
}

impl HandleTick for Ticker {
    fn handle_tick(&self, c: u32) {
        if c % 1000 == 0 {
            println!("... tick");
            unsafe { (*self.counter.get()) += 1 }
        }
    }
}