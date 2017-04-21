use chip::gpio::GPIOB;
use hal::rcc;
use hal::gpio;

// LED GREEN = PB0
// LED BLUE = PB7
// LED GREEN = PB14

pub fn led0() -> gpio::PinOutput {
    rcc::set_gpio_enabled(GPIOB, true);
    gpio::pin(GPIOB, 0).into_output()
}