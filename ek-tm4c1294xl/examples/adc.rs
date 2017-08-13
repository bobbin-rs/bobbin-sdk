#![no_std]
#![no_main]

#[macro_use]
extern crate ek_tm4c1294xl as board;

use board::hal::gpio::*;
use board::hal::adc::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("ADC Test");
       
    let adc0 = ADC0;
    let ch0 = ADC0_CH0;
    let a0 = PE3; // PE3 / AIN0

    adc0        
        .sysctl_set_enabled(true)
        .init();    
    a0.port().sysctl_set_enabled(true);
    a0.mode_ain(&ch0);

    // Disable Sequencer 0
    adc0.with_actss(|r| r.set_asen(0, 0));
    // Setup Sequencer 0 to read temperature sensor 
    adc0.with_ssctl(0, |r| r.set_ts(0, 0).set_ie(0, 1).set_end(0, 1));
    // Enable Sequencer 0
    adc0.with_actss(|r| r.set_asen(0, 1));

    loop {        
        // Initiate Sequencer 0
        adc0.set_pssi(|r| r.set_ss(0, 1));
        // Wait for Sequencer 0 Complete
        while adc0.ris().inr(0) == 0 {}
        // Clear Flag
        adc0.set_isc(|r| r.set_in(0, 1));
        // Read FIFO
        println!("{}", adc0.ssfifo(0).data().value());

        board::delay(1000);
    }
}
