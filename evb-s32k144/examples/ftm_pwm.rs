#![no_std]
#![no_main]

#[macro_use]
extern crate evb_s32k144 as board;

use board::hal::port::*;
//use board::hal::gpio::GpioExt;
use board::hal::ftm::*;
use board::hal::pcc;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    
    let led0 = board::led::LED0;

    println!("FTM Test");
    
    let ch = FTM0_CH2;
    let t0 = ch.periph();

    led0.mode_ftm(&ch);



    t0.pcc_set_enabled(true);
    t0.pcc_set_clock_source(pcc::ClockSource::SPLLDIV2);
    // t0.set_prescale(Prescale::Div64);
    t0.set_prescale(64);
    t0.set_modulo(2048);
    t0.set_count(0);

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

    // let mut n = 0;    
    // loop {
    //     while !t0.timer_overflow() {}
    //     t0.clr_timer_overflow();
    //     if n == 1_000_000 {
    //         led0.toggle_output();
    //         n = 0;
    //     }

    //     n += 1;
    //     //board::delay(1000);
    // }

}
