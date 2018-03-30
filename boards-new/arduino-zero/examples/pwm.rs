#![no_std]
#![no_main]

#[macro_use]
extern crate arduino_zero as board;
extern crate embedded_hal as hal;
extern crate examples;

use board::mcu::pin::*;
use board::mcu::gclk::{self, GCLK};
use board::mcu::tcc::*;

// LED0 = PWM / TIM2_CH2

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let led0 = PA17;

    let tcc = TCC0;
    let tcc_ch = TCC0_CH3;
    tcc.gate_enable();
    // led0.mode_wo_7(tcc);
    led0.connect_to(tcc);

    println!("Running PWM");    

    GCLK.set_clk(gclk::GenericClock::TCC0_TCC1, gclk::GenericClockGen::GClkGen3);
    println!("Clock Set");

    tcc_ch.pwm_up_low(0, 2000);

    let pwm = PwmCh::new(tcc_ch.into());
    let del = DelayTimer::new();
    
    let mut app = examples::pwm::Pwm::new(pwm, del, 10);
    app.run()
}

pub struct DelayTimer;

impl DelayTimer {
    pub fn new() -> Self { Self {} }
}

impl hal::blocking::delay::DelayMs<u16> for DelayTimer {
    fn delay_ms(&mut self, ms: u16) {
        board::delay(ms.into())
    }
}


pub struct PwmCh {
    tim_ch: TccCh,
}

impl PwmCh {
    pub fn new(tim_ch: TccCh) -> Self {
        Self { tim_ch }
    }
}

impl hal::PwmPin for PwmCh {
    type Duty = u16;
    fn disable(&mut self) {        
        self.tim_ch.periph.with_cc(self.tim_ch.index, |r| r.set_cc(0));        
    }
    fn enable(&mut self) {
        self.tim_ch.periph.with_cc(self.tim_ch.index, |r| r.set_cc(1));        
    }
    fn get_duty(&self) -> Self::Duty {
        self.tim_ch.compare()
    }
    fn get_max_duty(&self) -> Self::Duty {
        self.tim_ch.periph.per().per().value() as u16
    }
    fn set_duty(&mut self, duty: Self::Duty) {
        self.tim_ch.set_compare(duty.into());
    }
}