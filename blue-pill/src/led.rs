use hal::gpio::*;

pub const LED0: Pc13 = PC13;

pub fn init() {
    LED0.port().rcc_enable();
    LED0.mode_output();
}