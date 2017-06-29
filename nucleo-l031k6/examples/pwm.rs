#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_l031k6 as board;

use board::hal::tim::*;
use board::hal::gpio::{PinExt, ModeTim};

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let led0 = board::led::LED0;

    let ch = TIM2_CH2;
    
    led0.mode_tim(&ch).push_pull();

    let t = ch.periph();
    t.rcc_set_enabled(true);
    t.set_auto_reload(2000);

    ch.set_output_compare_mode(OcMode::Pwm1);
    ch.set_capture_compare_enabled(true);
    ch.set_capture_compare(0);

    t.set_enabled(true);

    println!("PWM Test");

    let max = 2000;
    let step = 20;
    let mut i: u32 = step; 
    let mut dir: bool = true;
    loop {        
        //t.set_capture_compare(ch.index(), i as u32);
        ch.set_capture_compare(i);
        
        if i == max { dir = false } else if i == 0 { dir = true }
        if dir {
            i += step 
        } else {
            i -= step;
        }
        board::delay(10);
    }
}
