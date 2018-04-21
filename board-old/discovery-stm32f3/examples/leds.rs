#![no_std]
#![no_main]

extern crate discovery_stm32f3 as board;

use board::led::*;
use board::hal::gpio::{GpioPin, DigitalOutput};

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    loop {
        for i in 0..8 {
            let led: GpioPin = match i {
                0 => LED_N.into(),
                1 => LED_NE.into(),
                2 => LED_E.into(),
                3 => LED_SE.into(),
                4 => LED_S.into(),
                5 => LED_SW.into(),
                6 => LED_W.into(),
                7 => LED_NW.into(),
                _ => unimplemented!(),
            };
            led.toggle_output();
            board::delay(100);            
        }
    }
}
