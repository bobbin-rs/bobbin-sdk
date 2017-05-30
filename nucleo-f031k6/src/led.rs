use chip::gpio::*;
use hal::rcc;
use chip::gpio::{Pb3, PB3};
use hal::gpio::{Mode, PinExt};

pub const LED0: Pb3 = PB3;

pub fn init() {
    rcc::set_gpio_enabled(&LED0.port(), true);
    LED0.set_mode(Mode::Output);
}