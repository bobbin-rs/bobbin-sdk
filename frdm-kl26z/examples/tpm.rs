#![no_std]
#![no_main]

#[macro_use]
extern crate frdm_kl26f as board;

use board::hal::port::ModeTpm;
use board::hal::tpm::*;

// Assume TPM bus clock is 60Mhz

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    
    println!("TPM Test");
    

    let ch = TPM0_CH2;
    let t0 = ch.periph();

    board::led::LED0.mode_tpm(&ch);

    t0.sim_set_src(0b01);
    t0.sim_set_enabled(true);
    t0.set_prescale(Prescale::Div128);
    t0.with_mod(|r| r.set_mod(2000));

    // Setup Edge PWM    
    ch.with_csc(|r| r.set_msb(1).set_msa(0).set_elsb(0).set_elsa(1));
    ch.set_value(1024);

    t0.set_clock(ClockMode::TpmClk);
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
    //     println!("Tick: {}", n);
    //     n += 1;
    //     //board::delay(1000);
    // }

}
