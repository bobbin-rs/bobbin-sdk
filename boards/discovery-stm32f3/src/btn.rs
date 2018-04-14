pub use mcu::pin::*;
pub use common::hal::btn::*;

pub const BTN0: Pa0 = PA0;

pub fn init() {
    BTN0.port().gate_enable();
    BTN0.mode_input().pull_down();
}

impl ::DiscoveryStm32f3 {
    pub fn btn0(&self) -> BtnHigh<GpioPin> { BtnHigh::new(PA0_PIN) }
}