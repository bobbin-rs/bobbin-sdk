use common::hal::delay::Delay;
use common::hal::led::*;

pub struct BlinkLeds<'a, LED: 'a + Led, DEL: Delay> {
    leds: &'a [LED],
    del: DEL,
    delay_ms: u32,
}

impl<'a, LED, DEL> BlinkLeds<'a, LED, DEL> 
where
    LED: 'a + Led,
    DEL: Delay
{
    pub fn new(leds: &'a [LED], del: DEL, delay_ms: u32) -> Self {
        Self { leds, del, delay_ms }
    }

    pub fn run(&self) -> ! {
        loop {
            for i in 0..self.leds.len() {
                self.leds[i].toggle();
                self.del.delay_ms(self.delay_ms);
            }
        }
    }
}