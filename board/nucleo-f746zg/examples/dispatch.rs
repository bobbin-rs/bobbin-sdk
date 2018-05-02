#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate nucleo_f746zg as board;
extern crate examples;

use board::prelude::*;
use board::mcu::irq::IRQ_TIM;
use board::mcu::tim_gen::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let mut sys = board::init();

    println!("Interrupt Dispatch Test");
    
    let tim = TIM3;

    tim.gate_enable();
    tim.set_auto_reload((sys.clk().clock_for(tim).as_u32() / 1000) as u16);    

    let h_irq = tim.irq_number_for(IRQ_TIM);
    let h = TimHandler::new(tim);
    let _h = sys.dispatcher_mut().register_handler(h_irq, &h).unwrap();

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

pub struct TimHandler {
    tim: TimGenPeriph,
    count: UnsafeCell<u32>,
}

impl TimHandler {
    pub fn new<T: Into<TimGenPeriph>>(tim: T) -> Self {
        TimHandler { tim: tim.into(), count: UnsafeCell::new(0) }
    }
    pub fn count(&self) -> u32 { 
        unsafe { *self.count.get() }
    }
    pub fn tick(&self) {
        unsafe { *self.count.get() += 1 }
    }
}

impl HandleIrq for TimHandler {
    fn handle_irq(&self, _: u8) {
        self.tim.set_sr(|r| r);
        self.tick()
    }
}

unsafe impl Sync for TimHandler {}