use chip::gpio::*;
use hal::gpio;

pub fn init() {
    gpio::pin(PA0).into_input(gpio::Pull::PullDown);
}

// BTN0 = PA0
pub fn btn0() -> gpio::PinInput {     
    gpio::pin_input(PA0)
}
