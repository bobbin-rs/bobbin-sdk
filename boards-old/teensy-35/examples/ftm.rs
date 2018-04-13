#![no_std]
#![no_main]

#[macro_use]
extern crate teensy_35 as board;

use board::hal::port::*;
use board::hal::ftm::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    
    let led0 = board::led::LED0;

    println!("FTM Test");
    

    let ch = FTM0_CH2;
    let t0 = ch.periph();

    led0.mode_ftm(&ch);

    t0.sim_set_enabled(true);
    t0.set_prescale(Prescale::Div128);
    t0.set_modulo(2048);

    // Setup Edge PWM    
    
    ch.set_pwmen(true);
    ch.with_csc(|r| r.set_msb(1).set_msa(0).set_elsb(0).set_elsa(1));
    ch.set_value(1024);

    
    t0.set_clock(ClockSource::SystemClk);

    println!("Clock Enabled");

    let max = 2000;
    let step = 20;
    let mut i: u32 = step; 
    let mut dir: bool = true;
    loop {        
        ch.set_value(i as u16);
        
        if i == max { dir = false } else if i == 0 { dir = true }
        if dir {
            i += step 
        } else {
            i -= step;
        }
        board::delay(10);
    }

}
