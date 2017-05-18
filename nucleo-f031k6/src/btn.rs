use hal::rcc;
use hal::gpio;

// BTN0 = PA12
pub fn btn0() -> gpio::PinInput { 
    let pin = ::chip::gpio::PA12;
    rcc::set_pin_enabled(pin, true);
    gpio::pin(pin).into_input(gpio::Pull::PullUp)
}