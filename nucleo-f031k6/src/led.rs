use chip::gpio::GPIOB;
use hal::rcc;
use driver::gpio;

pub type Led = gpio::PinOutput;

// LED @ D13 = PB3
pub fn led0() -> Led {
    rcc::set_gpio_enabled(GPIOB, true);
    gpio::pin(GPIOB, 3).into_output()
}