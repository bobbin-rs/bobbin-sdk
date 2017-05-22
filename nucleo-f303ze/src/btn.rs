use chip::gpio::*;
use hal::gpio;

pub fn init() {
    // let pin = PA12;
    // rcc::set_gpio_enabled(pin.port(), true);
    gpio::pin(PA12).into_input(gpio::Pull::PullUp);
}

// BTN0 = PC13
pub fn btn0() -> gpio::PinInput {     
    gpio::pin_input(PC13)
}
