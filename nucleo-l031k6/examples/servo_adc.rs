#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_l031k6 as board;

use board::common::bits::*;
use board::clock::CLK;
use board::hal::clock::Clock;
use board::hal::tim::*;
use board::hal::gpio::{PinExt, ModeTim};
use board::pin::*;
use board::hal::gpio::ModeAdc;
use board::hal::adc::*;
// Servo on D9 / PA8 / TIM2_CH1

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();

    let a0 = A0;    
    let a1 = A1;
    let d9 = D9;

    let adc_ch0 = ADC1_CH0;
    let adc_ch1 = ADC1_CH1;

    a0.port().rcc_set_enabled(true);
    a1.port().rcc_set_enabled(true);

    a0.mode_adc(&adc_ch0);
    // a1.mode_adc(&adc_ch1);

    let adc = adc_ch0.periph();
    adc
        .rcc_set_enabled(true)
        .init();


    let tim_ch = TIM2_CH1;    
    d9.mode_tim(&tim_ch).push_pull();

    let p = 10u32;    

    let t = tim_ch.periph();
    t.rcc_set_enabled(true);
    t.set_prescale(p as u16);
    // Scaled Clock 640KHz
    // Want 20ms interval = 50Hz    

    let period = t.clock(&CLK).unwrap() / (p * 50);
    let period_0 = (period / 20) - 320;
    let period_1 = (period / 10) + 320;
    println!("clock: {}",  t.clock(&CLK).unwrap());
    println!("period: {}", period);
    println!("period_0: {}", period_0);
    println!("period_1: {}", period_1);
    
    t.set_auto_reload(period);

    tim_ch.set_output_compare_mode(OcMode::Pwm1);
    tim_ch.set_capture_compare_enabled(true);
    tim_ch.set_capture_compare(0);

    t.set_enabled(true);

    println!("Servo ADC Test");

    // loop {}
    // let mut pos = period_0;
    let mut i = 0;
    loop {
        let v0: U12 = adc_ch0.analog_read();    
        let pos = period_0 + (v0.value() as u32);
        tim_ch.set_capture_compare(pos);
        if i == 1000 {
            println!("{} {}", v0, pos);
            i = 0;
        } else {
            i += 1;
        }
        board::delay(1);
    }
}
