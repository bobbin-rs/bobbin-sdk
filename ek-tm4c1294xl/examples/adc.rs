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

    loop {
        println!("{}", ch0.analog_read());

        board::delay(1000);
    }
}
