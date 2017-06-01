use hal::gpio::*;

pub const BTN0: Pa0 = PA0;

pub fn init() {
    BTN0.port().rcc_enable();
    BTN0.mode_input().pull_down();
}
