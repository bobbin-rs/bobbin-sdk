use chip::gpio::GPIOA;
use hal::rcc;
use driver::gpio;

// LED @ D13 = PA5
pub fn led0() -> gpio::PinOutput {
    rcc::set_gpio_enabled(GPIOA, true);
    gpio::pin(GPIOA, 5).into_output()
}