#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate nucleo_f746zg as board;
extern crate examples;

use board::prelude::*;
use board::mcu::tim_gen::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut sys = board::init();

    println!("Interrupt Dispatch Test");
    
    let tim = TIM3;

    tim.gate_enable();
    tim.set_auto_reload((sys.clk().clock_for(tim).as_u32() / 1000) as u16);    

    let mut buf = [0u8; 16];

    let h = TimHandler::new(tim, &mut buf);
    
    let _h = sys.dispatcher_mut().register_periph_handler(tim, &h).unwrap();

    sys.run(|sys| {
        tim
            .with_dier(|r| r.set_uie(1))
            .set_enabled(true);
        loop {
            println!("{} {}", tim.cnt(), h.count());
            sys.tick().delay(500);
        }
    })
}

use core::cell::UnsafeCell;

pub struct TimHandler<'a> {
    tim: TimGenPeriph,
    buf: &'a mut [u8],
    count: UnsafeCell<u32>,
}

impl<'a> TimHandler<'a> {
    pub fn new<T: Into<TimGenPeriph>>(tim: T, buf: &'a mut [u8]) -> Self {
        TimHandler { tim: tim.into(), buf, count: UnsafeCell::new(0) }
    }
    pub fn count(&self) -> u32 { 
        unsafe { *self.count.get() }
    }
    pub fn tick(&self) {
        unsafe { *self.count.get() += 1 }
    }
}

impl<'a> HandleIrq for TimHandler<'a> {
    fn handle_irq(&self, _: u8) {
        self.tim.set_sr(|r| r);
        self.tick()
    }
}

unsafe impl<'a> Sync for TimHandler<'a> {}