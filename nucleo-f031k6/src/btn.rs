use chip::gpio::*;
use hal::rcc;
use hal::gpio;

pub fn init() {
    let pin = PA12;
    rcc::set_gpio_enabled(pin.port(), true);
    gpio::pin((pin.port(), pin.index())).into_input(gpio::Pull::PullUp);
}

// BTN0 = PA12
pub fn btn0() -> gpio::PinInput { 
    let pin = PA12;
    gpio::pin_input((pin.port(), pin.index()))
}