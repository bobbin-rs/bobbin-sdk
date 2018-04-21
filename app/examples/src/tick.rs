#[allow(unused_imports)]

use embedded_hal::blocking::serial::Write;
use embedded_hal::blocking::delay::DelayMs;

pub struct Tick<OUT: Write<u8>, TIM: DelayMs<u16>> {
    out: OUT,
    tim: TIM,
    delay_ms: u16,
}

impl<OUT, TIM> Tick<OUT, TIM> 
where
    OUT: Write<u8>,
    TIM: DelayMs<u16>,
{
    pub fn new(out: OUT, tim: TIM, delay_ms: u16) -> Self {
        Self { out, tim, delay_ms }
    }

    pub fn run(&mut self) -> ! {
        loop {
            let _ = self.out.bwrite_all(b"Tick...\r\n");
            self.tim.delay_ms(self.delay_ms);
        }
    }
}