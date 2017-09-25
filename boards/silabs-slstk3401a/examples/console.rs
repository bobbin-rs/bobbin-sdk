#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]

#[macro_use]
extern crate silabs_slstk3401a as board;

use board::led::*;
use board::btn::*;
use board::hal::gpio::*;

// LED0 = PF4
// LED1 = PF5
// BTN0 = PF6
// BTN1 = PF7

pub const PIN: usize = 4;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();

    use board::hal::usart::*;;

    println!("{:?}", USART0.clkdiv());

    let refFreq = 40_000_000;
    let oversample = 16;
    let baudrate = 115200;

    let clkdiv  = 4 * refFreq + (oversample * baudrate) / 2;
    let clkdiv = clkdiv / (oversample * baudrate);
    let clkdiv = clkdiv - 4;
    let clkdiv = clkdiv * 64;    
    println!("clkdiv: {}", clkdiv);

    let mut i = 0;
    loop {
        println!("Hello, World {}", i);
        i += 1;
        board::delay(500);
    }
}
