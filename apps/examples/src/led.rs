use bobbin_hal::delay::*;
use bobbin_hal::led::*;

pub struct BlinkLed<LED: Led, DEL: Delay> {
    led: LED,
    del: DEL,
    delay_ms: u32,
}

impl<LED, DEL> BlinkLed<LED, DEL> 
where
    LED: Led,
    DEL: Delay
{
    pub fn new(led: LED, del: DEL, delay_ms: u32) -> Self {
        Self { led, del, delay_ms }
    }

    pub fn run(&self) -> ! {
        loop {
            self.led.toggle();
            self.del.delay_ms(self.delay_ms);
        }
    }
}