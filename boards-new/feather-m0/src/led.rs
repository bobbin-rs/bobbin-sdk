pub use common::led::*;
pub use mcu::pin::*;
pub use mcu::port::*;

pub const LED0: LedHigh<PortPin> = LedHigh::new(PA17_PIN);

pub fn init() {
    PA17.port().gate_enable();
    PA17.set_mode_output();
}

impl GetLed for ::FeatherM0 {
    fn get_led(&self, index: usize) -> &Led {
        match index {
            0 => &LED0,
            _ => unimplemented!()
        }
    }
    fn get_led_count(&self) -> usize { 1 }
}

impl ::FeatherM0 {
    pub fn led0(&self) -> LedHigh<PortPin> {
        LedHigh::new(PA17_PIN)
    }
}