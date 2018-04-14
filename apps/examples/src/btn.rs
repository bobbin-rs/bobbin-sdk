use common::delay::*;
use common::hal::led::*;
use common::hal::btn::*;

pub struct BtnExample<BTN: Btn, LED: Led, DEL: Delay> {
    btn: BTN,
    led: LED,
    del: DEL,
    delay_active: u32,
    delay_inactive: u32,
}

impl<BTN, LED, DEL> BtnExample<BTN, LED, DEL> 
where
    BTN: Btn,
    LED: Led,
    DEL: Delay
{
    pub fn new(btn: BTN, led: LED, del: DEL, delay_active: u32, delay_inactive: u32) -> Self {
        Self { btn, led, del, delay_active, delay_inactive }
    }

    pub fn run(&self) -> ! {
        loop {
            self.led.toggle();
            if self.btn.on() {
                self.del.delay_ms(self.delay_active);
            } else {
                self.del.delay_ms(self.delay_inactive);
            }
        }
    }
}