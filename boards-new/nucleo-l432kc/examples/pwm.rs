#![no_std]
#![no_main]

extern crate nucleo_l432kc as board;
extern crate embedded_hal as hal;
extern crate examples;

use board::mcu::pin::*;
use board::mcu::tim_gen::*;

// LED0 = PWM / TIM2_CH2

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    let led0 = PB3;

    let tim = TIM2;
    let tim_ch = TIM2_CH2;

    led0.port().gate_enable();    
    led0.connect_to(tim_ch);

    tim.gate_enable();
    tim.set_auto_reload(2000);

    tim_ch.set_output_compare_mode(OcMode::Pwm1);
    tim_ch.set_capture_compare_enabled(true);
    tim_ch.set_capture_compare(0);

    tim.set_enabled(true);

    let pwm = PwmCh::new(tim_ch.into());
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
    tim_ch: TimGenCh,
}

impl PwmCh {
    pub fn new(tim_ch: TimGenCh) -> Self {
        Self { tim_ch }
    }
}

impl hal::PwmPin for PwmCh {
    type Duty = u16;
    fn disable(&mut self) {
        self.tim_ch.set_capture_compare_enabled(false);
    }
    fn enable(&mut self) {
        self.tim_ch.set_capture_compare_enabled(true);
    }
    fn get_duty(&self) -> Self::Duty {
        self.tim_ch.capture_compare() as u16
    }
    fn get_max_duty(&self) -> Self::Duty {
        self.tim_ch.periph.auto_reload()
    }
    fn set_duty(&mut self, duty: Self::Duty) {
        self.tim_ch.set_capture_compare(duty.into());
    }
}