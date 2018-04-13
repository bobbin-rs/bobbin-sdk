#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_l432kc as board;

use board::hal::tim_bas::*;
use board::hal::RegisterPoll;
// use board::hal::nvic::{NvicEnabled, RegisterPoll};
use board::clock::*;
use core::ptr;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Millis Driver Test");    

    let t = TIM6;    
    t.rcc_enable();

    let prescale = t.clock(&CLK).unwrap() / 2000;
    println!("prescale: {}, {}", prescale, prescale as u16);
    t.set_prescale(prescale as u16);
    t.set_period(1);

    {

        let mut n = 1000;
        static mut COUNT: u32 = 0;

        let irq = t.irq_tim();
        println!("IRQ: {:?}", irq);

        let poll = || {
            if t.test_timeout() {
                t.clr_timeout();
                unsafe { 
                    ptr::write_volatile(&mut COUNT, ptr::read_volatile(&COUNT).wrapping_add(1));
                }
            }
        };

        let _guard = irq.register_poll(&poll);
        
        t.with_dier(|r| r.set_uie(1)).set_enabled(true);
        
        loop {        
            let c = unsafe { ptr::read_volatile(&COUNT) };
            if c == n {
                println!("Tick {}", c);
                n += 1000;
            }
            if c == 5000 {
                break;
            }
        }
        t.with_dier(|r| r.set_uie(0)).set_enabled(false);
        println!("Exiting First Example");
    }
    
    {
        let tc = TimBasCounter::new(t.into());

        let irq = t.irq_tim();
        let _guard = irq.register_poll(&tc);
        
        
        tc.enable();

        let mut n = 1000;
        loop {                
            let now = tc.counter();
            if now == n {
                println!("tick {}", now);
                n += 1000;
            }
            board::cortexm::wfi();
        }
    }
}
