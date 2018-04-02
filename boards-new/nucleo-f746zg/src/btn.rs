pub use mcu::pin::*;
pub use common::btn::*;

pub const BTN0: Pc13 = PC13;

pub fn init() {
    BTN0.port().gate_enable();
    BTN0.mode_input().pull_down();
}

impl ::NucleoF746zg {
    pub fn btn0(&self) -> BtnHigh<GpioPin> { BtnHigh::new(PC13_PIN) }
}