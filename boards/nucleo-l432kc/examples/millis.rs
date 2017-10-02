#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_l432kc as board;

use board::hal::tim_bas::*;
use board::clock::*;
use board::hal::nvic;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Millis Driver Test");    

    let t = TIM6;    
    t.rcc_enable();
    let irq = t.irq_tim();

    let prescale = t.clock(&CLK).unwrap() / 2000;
    println!("prescale: {}, {}", prescale, prescale as u16);
    t.set_prescaler(prescale as u16);
    t.set_reload(1);

    let tc = TimBasCounter::new(irq, t);
    tc.enable();
    
    // let _h = irq.wrap_handler(&tc);
    println!("irq: {:?}", irq);
    nvic::set_enabled(irq.irq_num() as usize, true);
    // irq.set_enabled(true);
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
