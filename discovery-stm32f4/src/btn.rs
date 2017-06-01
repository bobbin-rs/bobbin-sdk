use hal::gpio::*;
use hal::rcc;

pub const BTN0: Pa0 = PA0;

pub fn init() {
    rcc::enable(&BTN0.port());
    BTN0.mode_input().pull_down();
}
