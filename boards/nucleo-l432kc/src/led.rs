pub use bobbin_hal::led::*;
pub use bobbin_mcu::pin::Pin;
pub use bobbin_mcu::gate::GateEn;
pub use mcu::pin::*;
pub use mcu::gpio::*;
pub use mcu::ext::gpio::*;

pub const LED0: LedHigh<GpioPin> = LedHigh::new(PB3_PIN);

pub fn init() {
    PB3.port().gate_enable();
    PB3.mode_output();
}

impl GetLed for ::NucleoL432kc {
    fn get_led(&self, index: usize) -> &Led {
        match index {
            0 => &LED0,
            _ => unimplemented!()
        }
    }
    fn get_led_count(&self) -> usize { 1 }
}

impl ::NucleoL432kc {
    pub fn led0(&self) -> LedHigh<GpioPin> { LedHigh::new(PB3_PIN) }
}