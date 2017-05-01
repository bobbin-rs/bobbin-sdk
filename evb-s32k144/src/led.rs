use hal::port;
use pin;

// LED BLUE = PTD0
// LED Red = PTD15
// LED Green = PTD16

pub fn led_blue() -> port::PinOutput {
    pin::ptd0().into_output()
}

pub fn led_red() -> port::PinOutput {
    pin::ptd15().into_output()
}

pub fn led_green() -> port::PinOutput {
    pin::ptd16().into_output()
}