use pin;
use hal::port;

pub type Led = port::PinOutput;

// LED @ D13 = PA17
pub fn led0() -> Led {
    pin::pa17().into_digital_output()
}