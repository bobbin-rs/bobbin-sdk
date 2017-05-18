use pin;
use hal::gpio;

pub type Led = gpio::PinOutput;

// LED @ D13 = PB3
pub fn led0() -> Led {
    pin::pb3().into_output()
}