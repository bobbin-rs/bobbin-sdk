use chip::gpio::*;
use hal::rcc;
use hal::gpio;

// BTN0 = PA12
pub fn btn0() -> gpio::PinInput { 
    let pin = PA12;
    rcc::set_gpio_enabled(pin.port(), true);
    gpio::pin((pin.port(), pin.index())).into_input(gpio::Pull::PullUp)
}