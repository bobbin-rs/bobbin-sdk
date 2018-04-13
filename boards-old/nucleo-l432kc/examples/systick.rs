#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_l432kc as board;

use board::clock::*;
use board::hal::systick::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();    
    println!("Running Systick Example");

    let reload_value = (CLK.systick().unwrap() / 1000) - 1;
    println!("Setting reload_value to {}", reload_value);    
    SYSTICK.set_reload_value(reload_value);
    SYSTICK.set_current_value(reload_value);
    SYSTICK.set_enabled(true);

    let mut counter = 0u32;
    loop {
        if SYSTICK.count_flag() {
            counter += 1;
            if counter % 1000 == 0 {
                println!("Tick {}", counter);
                counter = counter.wrapping_add(1);
            }
        }        
    }
}
