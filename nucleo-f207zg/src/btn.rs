use chip::gpio::*;
use hal::gpio;

pub fn init() {
    gpio::pin(PC13).into_input(gpio::Pull::PullDown);
}

// BTN0 = PC13
pub fn btn0() -> gpio::PinInput {     
    gpio::pin_input(PC13)
}