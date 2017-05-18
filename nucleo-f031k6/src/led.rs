use chip::gpio::*;
use hal::gpio;

pub type Led = gpio::PinOutput;

pub fn init() {
    gpio::pin(PB3).into_output();
}

// LED @ D13 = PB3
pub fn led0() -> Led {
    gpio::pin_output(PB3)
}