use hal::gpio::*;
use hal::rcc;

pub const BTN0: Pa12 = PA12;

pub fn init() {
    rcc::enable(&BTN0.port());
    BTN0.mode_input().pull_up();
}
