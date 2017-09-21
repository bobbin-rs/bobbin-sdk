use hal::gpio::*;

pub const LED0: Pb13 = PB13;

pub fn init() {
    LED0.port().rcc_enable();
    LED0.mode_output();
}