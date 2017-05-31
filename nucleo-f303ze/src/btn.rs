use hal::rcc;
use hal::gpio::*;

pub const BTN0: Pc13 = PC13;

pub fn init() {
    rcc::enable(&BTN0.port());
    BTN0.mode_input().pull_up();
}
