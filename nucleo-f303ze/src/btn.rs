use chip::gpio::GPIOC;
use hal::rcc;
use hal::gpio;

// BTN0 = PC13
pub fn btn0() -> gpio::PinInput { 
    rcc::set_gpio_enabled(GPIOC, true);
    gpio::pin(GPIOC, 13).into_input(gpio::Pull::PullUp)
}