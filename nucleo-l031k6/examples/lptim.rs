#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_l031k6 as board;

use board::hal::gpio::*;
use board::led::LED0;
use board::hal::lptim::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("LPTIM Test");


    // Timer Source = 32MHz
    // Prescaler - Divide by 128
    // Timer Clock 250KHz
    // Reload = 50000
    // Overflow @ 5Hz

    let t = LPTIM;
    // Select LSE Clock as timer source
    t.rcc_set_sel(0b00);
    t.rcc_set_enabled(true);    
    t.set_cfgr(|r| r.set_presc(0b111));
    t.set_cr(|r| r.set_enable(1));    

    t.set_arr(|r| r.set_arr(50000));
    while t.isr().arrok() == 0 {}

    t.set_icr(|r| r.set_arrmcf(1));
    
    t.set_cr(|r| r.set_cntstrt(1).set_enable(1));

    let mut n = 0;
    loop {
        while t.isr().arrm() == 0 {}
        t.set_icr(|r| r.set_arrmcf(1));
        LED0.toggle_output();
        if n == 4 {
            println!("tick..");
            n = 0;
        } else {
            n += 1;
        }
    }
}

