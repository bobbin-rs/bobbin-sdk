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

    println!("PWM Test");
    
    let ch = FTM0_CH2;
    let t0 = ch.periph();

    led0.mode_ftm(&ch);

    t0
        .pcc_set_clock_source(pcc::ClockSource::SPLLDIV2)
        .pcc_set_enabled(true)
        .set_prescale(64);

    ch.pwm_high(0, 2048);

    println!("PWM Enabled");

        
    let max = 2000u16;
    let step = 20u16;
    let mut i: u16 = step; 
    let mut dir: bool = true;
    loop {       
        ch.set_compare(i);
        
        if i == max { dir = false } else if i == 0 { dir = true; board::delay(1000); }
        if dir {
            i += step 
        } else {
            i -= step;
        }
        board::delay(10);
    }
}
