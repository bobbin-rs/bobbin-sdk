use hal::gpio::*;

pub const BTN0: Pb0 = PB0;

pub fn init() {
    BTN0.port().rcc_enable();
    BTN0.mode_input().pull_down();
}
