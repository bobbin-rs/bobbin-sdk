#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate arduino_zero as board;

use board::hal::gclk;
use board::hal::tcc::*;
use board::hal::port::{PmEnabled, ModeWo7};

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let led0 = board::led::LED0;

    let ch = TCC0_CH3;
    let tcc = ch.periph();
    tcc.pm_set_enabled(true);
    led0.mode_wo_7(tcc);

    println!("Running PWM");    

    gclk::set_clk(gclk::GenericClock::TCC0_TCC1, gclk::GenericClockGen::GClkGen3);
    println!("Clock Set");

    // tcc.set_period(2000);

    // tcc.with_per(|r| r.set_per(2000));
    println!("Period Set");
    // tcc.with_cc(ch.index(), |r| r.set_cc(1));
    // ch.set_compare(1);
    println!("CC Set");
    // tcc.with_wave(|r| r.set_wavegen(0x02));
    // tcc.with_ctrla(|r| r.set_cpten(ch.index(), 1).set_enable(1));
    // tcc.with_ctrla(|r| r.set_enable(1));

    println!("Setup Complete");

    let max = 2000u16;
    let step = 40u16;
    let mut i: u16 = step; 
    let mut dir: bool = true;
    
    ch.pwm_up_low(step, max);

    loop {        
        // tcc.with_cc(ch.index(), |r| r.set_cc(i));
        ch.set_compare(i);
        
        if i == max { dir = false } else if i == 0 { dir = true; board::delay(500) }
        if dir {
            i += step 
        } else {
            i -= step;
        }
        board::delay(1);
    }
}