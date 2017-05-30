use chip::gpio::*;
use hal::gpio::*;
use hal::rcc;

pub const BTN0: Pa12 = PA12;

pub fn init() {
    rcc::set_gpio_enabled(&BTN0.port(), true);
    BTN0.set_mode(Mode::Input).set_pull(Pull::PullUp);
}
