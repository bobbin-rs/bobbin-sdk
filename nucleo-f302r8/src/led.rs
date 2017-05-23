use chip::gpio::*;
use hal::gpio;

pub type Led = gpio::PinOutput;

pub fn init() {
    gpio::pin(PB13).into_output();
}

// LED @ D13 = PB13
pub fn led0() -> Led {
    gpio::pin_output(PB13)
}
