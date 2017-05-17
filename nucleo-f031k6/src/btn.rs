use chip::pins;
use hal::rcc;
use hal::gpio;

// BTN0 = PA12
pub fn btn0() -> gpio::PinInput { 
    rcc::set_gpio_enabled(pins::PA12.0, true);
    gpio::pin(pins::PA12).into_input(gpio::Pull::PullUp)
}