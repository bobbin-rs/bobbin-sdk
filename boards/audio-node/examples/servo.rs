#![no_std]
#![no_main]

#[macro_use]
extern crate audio_node as board;

use board::clock::CLK;
use board::hal::clock::Clock;
use board::hal::tim::*;
use board::hal::gpio::{PinExt, ModeTim};
use board::btn::BTN0;
// Servo on D9 / PA8 / TIM2_CH1

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let d9 = board::pin::D9;

    let ch = TIM2_CH1;    
    d9.mode_tim(&ch).push_pull();

    let p = 10u32;    

    let t = ch.periph();
    t.rcc_set_enabled(true);
    t.set_prescaler(p as u16);
    // Scaled Clock 640KHz
    // Want 20ms interval = 50Hz    

    let period = t.clock(&CLK).unwrap() / (p * 50);
    let period_0 = (period / 20) - 320;
    let period_1 = (period / 10) + 320;
    println!("clock: {}",  t.clock(&CLK).unwrap());
    println!("period: {}", period);
    println!("period_0: {}", period_0);
    println!("period_1: {}", period_1);
    
    t.set_auto_reload(period);

    ch.set_output_compare_mode(OcMode::Pwm1);
    ch.set_capture_compare_enabled(true);
    ch.set_capture_compare(0);

    t.set_enabled(true);

    println!("PWM Test");

    // loop {}
    let mut pos = period_0;
    let step = 8;
    let mut n = 0;
    loop {        
        if n == 200 {
            n = 0;
            println!("{}", pos);
        } else {
            n += 1;
        }
        if BTN0.input() {
            // released
            if pos < period_1 {                
                pos += step;            
            }
        } else {
            // pressed
            if pos > period_0 {
                pos -= step;
            }
        }
        ch.set_capture_compare(pos);
        board::delay(1);
    }
}
