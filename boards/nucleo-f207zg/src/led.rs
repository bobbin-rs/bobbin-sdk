use hal::gpio::*;

pub const LED0: Pb0 = PB0;
pub const LED1: Pb7 = PB7;
pub const LED2: Pb14 = PB14;

pub fn init() {
    LED0.port().rcc_enable();
    LED0.mode_output();

    LED1.port().rcc_enable();
    LED1.mode_output();

    LED2.port().rcc_enable();
    LED2.mode_output();
}