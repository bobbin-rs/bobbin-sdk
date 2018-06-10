#![no_std]
#![no_main]

#[macro_use]
extern crate discovery_stm32f429i as board;
extern crate embedded_hal as hal;
extern crate examples;

use board::mcu::pin::*;
use board::mcu::tim_gen::*;
use board::prelude::*;
use board::mcu::tim_gen::TimGenCh;
use board::mcu::ext::tim_gen::OcMode;

// PWM output on PE9 / TIM1_CH1

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init().run(|brd| {
        let pa5 = PA5;

        let tim = TIM2;
        let tim_ch = TIM2_CH1;

        pa5.port().gate_enable();

        pa5.connect_to(tim_ch);
        pa5.push_pull();

        tim.gate_enable();
        tim.set_auto_reload(2000);

        tim_ch.set_output_compare_mode(OcMode::Pwm1);
        tim_ch.set_capture_compare_enabled(true);
        // tim_ch.set_capture_compare_selection(CcSelect::Output);
        tim_ch.set_capture_compare(0);
        tim_ch.set_capture_compare_enabled(true);

        tim.set_main_output_enable(true);
        tim.set_enabled(true);


        let pwm = PwmCh::new(tim_ch.into());

        let mut app = examples::pwm::Pwm::new(pwm, brd.tick(), 10);
        app.run()
    })
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