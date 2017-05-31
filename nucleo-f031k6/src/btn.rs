use chip::gpio::*;
use hal::rcc::{RCC, RccExt};
use hal::gpio::{PinExt};

pub const BTN0: Pa12 = PA12;

pub fn init() {
    RCC.set_enabled(&BTN0.port(), true);
    BTN0.mode_input().pull_up();
}
