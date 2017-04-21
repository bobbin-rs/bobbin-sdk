use chip::gpio::GPIOA;
use hal::rcc;
use hal::gpio;

// BTN0 = PA12
pub fn btn0() -> gpio::PinInput { 
    rcc::set_gpio_enabled(GPIOA, true);
    gpio::pin(GPIOA, 12).into_input(gpio::Pull::PullUp)
}