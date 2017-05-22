use chip::gpio::*;
use hal::gpio;

pub type Led = gpio::PinOutput;

pub fn init() {
    gpio::pin(PA5).into_output();
}

// LED @ D13 = PA5
pub fn led0() -> Led {
    gpio::pin_output(PA5)
}
