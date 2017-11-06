#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_f746zg as board;

use board::hal::gpio::*;
use board::hal::tim_adv::*;
use board::hal::clock::Clock;
use board::clock::CLK;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();

    let tim = TIM1;
    let tim_ch = TIM1_CH1;
    let tim_pin = PA8;


    tim.rcc_enable();
    tim_pin.port().rcc_enable();

    tim_pin.mode_output();
    tim_pin.mode_tim(&tim_ch);

    let frequency = 9_000_000;
    let tclk = tim.clock(&CLK).unwrap();
    let period = (tclk / frequency) - 1;
    let compare = ((period + 1) / 2) - 1;
    // let compare = 3;
    println!("tclk: {} freq: {} period: {} comp: {}", tclk, frequency, period, compare);

    // TIM1 Input Clock = 216MHz
    // TIM1 Prescaler = 24
    // TIM1 Output = 9MHz

    /* TCLK (PCLK * 2) */
    // int tclk  = DCMI_TIM_PCLK_FREQ() * 2;

    // /* Timer base configuration */
    // TIMHandle.Instance          = DCMI_TIM;
    // TIMHandle.Init.Period       = period;
    // TIMHandle.Init.Prescaler    = 0;
    // TIMHandle.Init.ClockDivision = 0;
    // TIMHandle.Init.CounterMode   = TIM_COUNTERMODE_UP;

    // /* Timer channel configuration */
    // TIM_OC_InitTypeDef TIMOCHandle;
    // TIMOCHandle.Pulse       = period/2;
    // TIMOCHandle.OCMode      = TIM_OCMODE_PWM1;
    // TIMOCHandle.OCPolarity  = TIM_OCPOLARITY_HIGH;
    // TIMOCHandle.OCFastMode  = TIM_OCFAST_DISABLE;
    // TIMOCHandle.OCIdleState = TIM_OCIDLESTATE_RESET;

    tim.set_enabled(false);
    tim.with_cr1(|r| r.set_cms(0b00).set_dir(0));
    tim.with_bdtr(|r| r.set_moe(1));
    tim.set_auto_reload(period);    
    tim_ch.set_output_compare_mode(OcMode::Pwm1);
    tim_ch.set_preload_enable(true);
    tim_ch.set_capture_compare(compare);
    tim_ch.set_capture_compare_enabled(true);
    tim.set_update_event();
    tim.set_enabled(true);
    println!("Timer Running");

    // loop {
    //     tim_ch.set_output_compare_mode(OcMode::ForceInactive);
    //     board::delay(100);
    //     tim_ch.set_output_compare_mode(OcMode::ForceActive);
    //     board::delay(100);
    // }
    loop {}
}
