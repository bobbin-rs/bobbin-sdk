use hal::gpio;
use pin;

// Led0 => (GpioF, 0),
// Led1 => (GpioF, 4),
// Led2 => (GpioN, 0),
// Led3 => (GpioN, 1),

pub fn init() {
}

pub fn led0() -> gpio::PinOutput {
    pin::pf0().into_output()
}
pub fn led1() -> gpio::PinOutput {
    pin::pf4().into_output()
}
pub fn led2() -> gpio::PinOutput {
    pin::pn0().into_output()
}
pub fn led3() -> gpio::PinOutput {
    pin::pn1().into_output()
}