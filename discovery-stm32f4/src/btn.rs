use chip::gpio::GPIOA;
use hal::rcc;
use hal::gpio;

// BTN0 = PA0
pub fn btn0() -> gpio::PinInput { 
    rcc::set_gpio_enabled(GPIOA, true);
    gpio::pin(GPIOA, 0).into_input(gpio::Pull::PullUp)
}