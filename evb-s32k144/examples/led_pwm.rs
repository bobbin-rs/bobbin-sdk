#![no_std]
#![no_main]

#[macro_use]
extern crate evb_s32k144 as board;

// use board::led::LedPwmSet;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();

    println!("LED PWM Test");

    let led = board::led::LED_RGB;
    led.init();

    let max = 1000u16;
    let step = 10u16;
    let mut i: u16 = step; 
    let mut dir: bool = true;
    led.start();
    loop {       
        led.set((i, 1000-i, 0));
        
        if i == max { dir = false } else if i == 0 { dir = true; board::delay(1000); }
        if dir {
            i += step 
        } else {
            i -= step;
        }
        board::delay(10);
    }
}
