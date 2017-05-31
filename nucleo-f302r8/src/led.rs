use hal::gpio::*;
use hal::rcc;

pub const LED0: Pb13 = PB13;

pub fn init() {
    rcc::enable(&LED0.port());
    LED0.mode_output();
}
