//! This example shows the usage of a PWM output, gradually fading a LED
//! in and out.
#[allow(unused_imports)]

use embedded_hal::PwmPin;
use embedded_hal::blocking::delay::DelayMs;

pub struct Pwm<PIN: PwmPin, TIM: DelayMs<u16>> {
    pin: PIN,
    tim: TIM,
    delay_ms: u16,
}

impl<PIN, TIM> Pwm<PIN, TIM> 
where
    PIN: PwmPin<Duty=u16>,
    TIM: DelayMs<u16>,
{
    pub fn new(pin: PIN, tim: TIM, delay_ms: u16) -> Self {
        Self { pin, tim, delay_ms }
    }

    pub fn run(&mut self) -> ! {

        let max = self.pin.get_max_duty();
        let step = 20;
        let mut i = step; 
        let mut dir: bool = true;
        loop {        
            self.pin.set_duty(i);
            
            if i == max { dir = false } else if i == 0 { dir = true }
            if dir {
                i += step 
            } else {
                i -= step;
            }
            self.tim.delay_ms(self.delay_ms);
        }        
    }
}