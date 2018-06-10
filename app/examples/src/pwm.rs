//! This example shows the usage of a PWM output, gradually fading a LED
//! in and out.
#[allow(unused_imports)]

use embedded_hal::PwmPin;
use bobbin_hal::delay::*;

pub struct Pwm<PIN: PwmPin, TIM: Delay> {
    pin: PIN,
    tim: TIM,
    delay_ms: u32,
}

impl<PIN, TIM> Pwm<PIN, TIM> 
where
    PIN: PwmPin<Duty=u16>,
    TIM: Delay,
{
    pub fn new(pin: PIN, tim: TIM, delay_ms: u32) -> Self {
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