#![no_std]
#![no_main]

#[macro_use]
extern crate nero_f7 as board;

use board::clock::CLK;
use board::hal::clock::Clock;
use board::hal::tim::*;
use board::hal::gpio::*;

// CH5 - PB0 / TIM3_CH3 / AF_2
// CH6 - PB1 / TIM3_CH4 / AF_2

// Servo period 20ms = 50hz

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Servo Test");


    let t = TIM3;
    let ch5 = TIM3_CH3;
    let ch6 = TIM3_CH4;

    PB0.mode_tim(&ch5).push_pull();
    PB1.mode_tim(&ch6).push_pull();

    t.rcc_set_enabled(true);

    let tim_clk = t.clock(&CLK).unwrap();
    let tim_pre = 36;
    let tim_hz = tim_clk / tim_pre;
    let tim_per = tim_hz / 50;

    let max = tim_per / 10;
    let min = tim_per / 20;
    let diff = max - min;
    let steps = 1000;
    let step = diff / steps;

    t.set_prescale(tim_pre as u16);
    t.set_auto_reload(tim_per);
    

    ch5.set_output_compare_mode(OcMode::Pwm1);
    ch5.set_capture_compare_enabled(true);
    ch5.set_capture_compare(0);

    ch6.set_output_compare_mode(OcMode::Pwm1);
    ch6.set_capture_compare_enabled(true);
    ch6.set_capture_compare(0);

    
    t.set_enabled(true);

    println!("tim_clk: {}", tim_clk);
    println!("tim_pre: {}", tim_pre);
    println!("tim_hz: {}", tim_hz);
    println!("tim_per: {}", tim_per);
    println!("max: {}", max);
    println!("min: {}", min);
    println!("diff: {}", diff);

    loop {
        for i in 0..100 {
            let n = i * 10 * step + min;
            ch5.set_capture_compare(n);
            ch6.set_capture_compare(n);        
            board::delay(10);
        }
        for i in 0..100 {
            let n = (100 - i) * 10 * step + min;
            ch5.set_capture_compare(n);
            ch6.set_capture_compare(n);        
            board::delay(10);
        }
    }
}
