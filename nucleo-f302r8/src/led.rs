use chip::gpio::GPIOB;
use hal::rcc;
use hal::gpio;

// LED @ D13 = PB13
pub fn led0() -> gpio::PinOutput {
    rcc::set_gpio_enabled(GPIOB, true);
    gpio::pin(GPIOB, 13).into_output()
}