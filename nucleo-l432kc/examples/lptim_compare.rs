#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_l432kc as board;

use board::hal::gpio::*;
use board::hal::clock::Clock;
use board::clock::CLK;
use board::hal::lptim::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("LPTIM Test");

    // Timer Source = 32MHz
    // Prescaler - Divide by 128
    // Timer Clock 250KHz
    // Reload = 62,500
    // Overflow @ 4Hz

    let t = LPTIM;
    // Select LSE Clock as timer source
    t.rcc_set_sel(0b00);
    t.rcc_set_enabled(true);    
    
    let prescale = 128;
    // 1/4 second to fit in 16 bit prescaler
    let period = (t.clock(&CLK).unwrap() / 128) >> 2;    

    println!("prescale: {}, period: {}", prescale, period);

    t
        .set_prescale(prescale)
        .set_compare(prescale >> 1)
        .start(period as u16);
        
    loop {
        t.clr_compare_flag().wait_compare_flag();
        println!("compare");
        t.clr_timeout_flag().wait_timeout_flag();
        println!("timeout");
    }

}

