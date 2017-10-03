#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_l432kc as board;

use board::hal::tim_bas::*;
use board::hal::nvic;
use board::clock::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Millis Driver Test");    

    let t = TIM6;    
    t.rcc_enable();
    let irq = t.irq_tim();

    let prescale = t.clock(&CLK).unwrap() / 2000;
    println!("prescale: {}, {}", prescale, prescale as u16);
    t.set_prescale(prescale as u16);
    t.set_period(1);

    let tc = TimBasCounter::new(t.into());
    irq.register_poll(&tc); 
    nvic::set_enabled(irq.irq_num() as usize, true);    
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
