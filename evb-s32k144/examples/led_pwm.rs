#![no_std]
#![no_main]

#[macro_use]
extern crate evb_s32k144 as board;

use board::led::LedPwmSet;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    

    println!("PWM Test");

    let led0 = board::led::LED_PWM0;
    let led1 = board::led::LED_PWM1;
    let led2 = board::led::LED_PWM2;
    led0.init();
    led1.init();
    led2.init();
    led0.start();
    led1.start();
    led2.start();
    let max = 1000u16;
    let step = 10u16;
    let mut i: u16 = step; 
    let mut dir: bool = true;
    loop {       
        led0.set(i);
        led1.set(1000-i);
        // led2.set(i + 500 % 1000);
        
        if i == max { dir = false } else if i == 0 { dir = true; board::delay(1000); }
        if dir {
            i += step 
        } else {
            i -= step;
        }
        board::delay(10);
    }
    
    // let led0 = board::led::LED0;
    // let ch = FTM0_CH2;
    // let t0 = ch.periph();

    // led0.mode_ftm(&ch);

    // t0
    //     .pcc_set_clock_source(pcc::ClockSource::SPLLDIV2)
    //     .pcc_set_enabled(true)
    //     .set_prescale(64);


    // // LED is active low, use pwm_low

    // ch.pwm_low(0, 2048);

    // println!("PWM Enabled, Pauses at Zero");

    // loop {}
}
