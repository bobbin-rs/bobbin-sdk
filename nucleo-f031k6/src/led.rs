use hal::rcc;
use hal::gpio;

pub type Led = gpio::PinOutput;

// LED @ D13 = PB3
pub fn led0() -> Led {
    let pin = ::chip::gpio::PB3;
    rcc::set_pin_enabled(pin, true);
    gpio::pin(pin).into_output()
}