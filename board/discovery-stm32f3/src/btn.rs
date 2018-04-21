use ::prelude::*;
pub use mcu::pin::*;

pub const BTN0: Pa0 = PA0;

pub fn init() {
    BTN0.port().gate_enable();
    BTN0.mode_input().pull_down();
}

impl ::Board {
    pub fn btn0(&self) -> BtnHigh<GpioPin> { BtnHigh::new(PA0_PIN) }
}