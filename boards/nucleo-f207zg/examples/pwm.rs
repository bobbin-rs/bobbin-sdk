#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f207zg as board;

use board::hal::tim::*;
use board::hal::gpio::{PinExt, ModeTim};

// PWM output on PB0 / TIM3_CH3 = AF_2

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let led0 = board::led::LED0;

    let ch = TIM3_CH3;
    
    led0.mode_tim(&ch).push_pull();

    let t = ch.periph();
    t.rcc_set_enabled(true);

    println!("PWM Test");

    let max = 2000u16;
    let step = 20u16;
    let mut i: u16 = step; 
    let mut dir: bool = true;

    ch.pwm_up_high(i, max);

    loop {        
        ch.set_compare(i);
        
        if i == max { dir = false } else if i == 0 { dir = true; board::delay(500) }
        if dir {
            i += step 
        } else {
            i -= step;
        }
        board::delay(10);
    }
}
