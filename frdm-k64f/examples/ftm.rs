#![no_std]
#![no_main]

#[macro_use]
extern crate frdm_k64f as board;

use board::hal::port::*;
use board::hal::gpio::GpioExt;
use board::hal::ftm::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    
    let led0 = board::led::LED0.gpio_pin();

    println!("FTM Test");
    

    let ch = FTM0_CH2;
    let t0 = ch.periph();

    t0.sim_set_enabled(true);
    t0.set_prescale(Prescale::Div128);
    t0.set_modulo(8192 * 2);
    t0.set_clock(ClockSource::SystemClk);

    println!("Clock Enabled");

    let mut n = 0;    
    loop {
        while !t0.timer_overflow() {}
        t0.clr_timer_overflow();
        if n == 1_000_000 {
            led0.toggle_output();
            n = 0;
        }

        n += 1;
        //board::delay(1000);
    }

}
