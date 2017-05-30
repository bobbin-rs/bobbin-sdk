use chip::gpio::*;
use hal::rcc::{RCC, RccExt};
use hal::gpio::{Mode, Pull, PinExt};

pub const BTN0: Pa12 = PA12;

pub fn init() {
    RCC.set_enabled(&BTN0.port(), true);
    BTN0.set_mode(Mode::Input).set_pull(Pull::PullUp);
}
