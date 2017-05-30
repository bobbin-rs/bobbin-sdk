use chip::gpio::*;
use chip::gpio::{Pb3, PB3};
use hal::rcc::{RCC, RccExt};
use hal::gpio::{Mode, PinExt};

pub const LED0: Pb3 = PB3;

pub fn init() {
    RCC.set_enabled(&LED0.port(), true);
    LED0.set_mode(Mode::Output);
}