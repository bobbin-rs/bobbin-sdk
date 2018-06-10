//use ::prelude::*;

pub use bobbin_hal::led::*;
pub use mcu::pin::*;
pub use mcu::gpio::*;
use mcu::bobbin_mcu::Pin;
use mcu::bobbin_mcu::prelude::GateEn;

pub const LED0: LedHigh<GpioPin> = LedHigh::new(PG13_PIN);
pub const LED1: LedHigh<GpioPin> = LedHigh::new(PG14_PIN);


pub fn init() {
    PG13.port().gate_enable();
    PG13.mode_output();

    PG14.port().gate_enable();
    PG14.mode_output();
}

impl GetLed for ::Board {
    fn get_led(&self, index: usize) -> &Led {
        match index {
            0 => &LED0,
            1 => &LED1,
            _ => unimplemented!()
        }
    }
    fn get_led_count(&self) -> usize { 2 }
}

impl ::Board {
    pub fn led0(&self) -> LedHigh<GpioPin> { LedHigh::new(PG13_PIN) }
    pub fn led1(&self) -> LedHigh<GpioPin> { LedHigh::new(PG14_PIN) }
}