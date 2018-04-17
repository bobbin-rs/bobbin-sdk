pub use bobbin_hal::led::*;
pub use mcu::pin::*;
pub use mcu::gpio::*;

pub const LED0: LedHigh<GpioPin> = LedHigh::new(PB0_PIN);
pub const LED1: LedHigh<GpioPin> = LedHigh::new(PB7_PIN);
pub const LED2: LedHigh<GpioPin> = LedHigh::new(PB14_PIN);

pub fn init() {
    PB0.port().gate_enable();
    PB0.mode_output();

    PB7.port().gate_enable();
    PB7.mode_output();

    PB14.port().gate_enable();
    PB14.mode_output();
}

impl GetLed for ::NucleoF746zg {
    fn get_led(&self, index: usize) -> &Led {
        match index {
            0 => &LED0,
            1 => &LED1,
            2 => &LED2,
            _ => unimplemented!()
        }
    }
    fn get_led_count(&self) -> usize { 3 }
}

impl ::NucleoF746zg {
    pub fn led0(&self) -> LedHigh<GpioPin> { LedHigh::new(PB0_PIN) }
    pub fn led1(&self) -> LedHigh<GpioPin> { LedHigh::new(PB7_PIN) }
    pub fn led2(&self) -> LedHigh<GpioPin> { LedHigh::new(PB14_PIN) }
}