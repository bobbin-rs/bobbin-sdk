use hal::gpio::*;
use hal::rcc;

pub const LED0: Pb0 = PB0;
pub const LED1: Pb7 = PB7;
pub const LED2: Pb14 = PB14;

pub fn init() {
    rcc::enable(&LED0.port());
    LED0.mode_output();

    rcc::enable(&LED1.port());
    LED1.mode_output();

    rcc::enable(&LED2.port());
    LED2.mode_output();
}
