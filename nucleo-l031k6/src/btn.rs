use hal::gpio::*;
use common::Pin;

pub const BTN0: Pa12 = PA12;

pub fn init() {
    BTN0.port().rcc_enable();
    BTN0.mode_input().pull_up();
}
