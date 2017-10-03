#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_l432kc as board;

use board::hal::tim_bas::*;
use board::hal::nvic::NvicEnabled;
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

    let mut n = 1000;
    static mut COUNT: u32 = 0;

    let irq = t.irq_tim();
    irq.register_poll(&|| {
        if t.test_timeout() {
            t.clr_timeout();
            unsafe { 
                ptr::write_volatile(&mut COUNT, ptr::read_volatile(&COUNT).wrapping_add(1));
            }
        }
    });
    irq.nvic_enable(); 
    t.with_dier(|r| r.set_uie(1)).set_enabled(true);
    
    loop {        
        let c = unsafe { ptr::read_volatile(&COUNT) };
        // println!("c: {}", c);        
        if c == n {
            println!("Tick {}", c);
            n += 1000;
        }
        // board::delay(100);
    }
    
    let tc = TimBasCounter::new(t.into());

    let irq = t.irq_tim();
    irq.register_poll(&tc);
    irq.nvic_enable(); 
    
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
