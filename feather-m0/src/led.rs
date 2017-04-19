use chip::port::PORTA;
use driver::port;

pub type Led = port::PinOutput;

// LED @ D13 = PA17
pub fn led0() -> Led {
    port::pin(PORTA, 17).into_digital_output()
}