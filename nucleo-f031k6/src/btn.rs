use pin;
use hal::gpio;

// BTN0 = PA12
pub fn btn0() -> gpio::PinInput { 
    pin::pa12().into_input(gpio::Pull::PullUp)
}