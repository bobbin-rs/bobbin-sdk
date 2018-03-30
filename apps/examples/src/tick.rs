#[allow(unused_imports)]

use embedded_hal::blocking::delay::DelayMs;
use core::fmt::Write;

pub struct Tick<OUT: Write, TIM: DelayMs<u16>> {
    out: OUT,
    tim: TIM,
    delay_ms: u16,
}

impl<OUT, TIM> Tick<OUT, TIM> 
where
    OUT: Write,
    TIM: DelayMs<u16>,
{
    pub fn new(out: OUT, tim: TIM, delay_ms: u16) -> Self {
        Self { out, tim, delay_ms }
    }

    pub fn run(&mut self) -> ! {
        loop {
            let _ = self.out.write_str("Tick...\r\n");
            self.tim.delay_ms(self.delay_ms);
        }
    }
}