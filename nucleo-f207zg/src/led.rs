use chip::gpio::*;
use hal::gpio;

pub type Led = gpio::PinOutput;

pub fn init() {
    gpio::pin(PB0).into_output();
    gpio::pin(PB7).into_output();
    gpio::pin(PB14).into_output();
}

pub fn led0() -> Led {
    gpio::pin_output(PB0)
}

pub fn led1() -> Led {
    gpio::pin_output(PB7)
}

pub fn led2() -> Led {
    gpio::pin_output(PB14)
}