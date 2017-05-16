use chip::pins;
use hal::rcc;
use hal::gpio;

pub type Led = gpio::PinOutput;

// LED @ D13 = PB3
pub fn led0() -> Led {
    rcc::set_gpio_enabled(pins::PB3.0, true);
    gpio::pin(pins::PB3).into_output()
}