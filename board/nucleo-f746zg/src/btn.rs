use bobbin_mcu::pin::Pin;
use bobbin_mcu::gate::GateEn;
pub use mcu::pin::*;
pub use bobbin_hal::btn::*;

pub const BTN0: Pc13 = PC13;

pub fn init() {
    BTN0.port().gate_enable();
    BTN0.mode_input().pull_down();
}

impl ::Board {
    pub fn btn0(&self) -> BtnHigh<GpioPin> { BtnHigh::new(PC13_PIN) }
}