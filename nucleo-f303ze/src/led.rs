use hal::gpio::*;
use hal::rcc;

pub const LED0: Pa5 = PA5;

pub fn init() {
    rcc::enable(&LED0.port());
    LED0.mode_output();
}
