use stm32_common::hal::gpio;
use chip::gpio::*;
use rcc;

pub use stm32_common::hal::gpio::{Pull, PinUnknown, PinOutput, PinInput, PinAltFn, PinAnalog};

pub fn pin<P: Pin>(pin: P) -> PinUnknown {
    rcc::set_gpio_enabled(pin.port(), true);
    gpio::pin((pin.port(), pin.index()))
}

pub fn pin_output<P: Pin>(pin: P) -> PinOutput {
    rcc::set_gpio_enabled(pin.port(), true);
    gpio::pin_output((pin.port(), pin.index()))
}

pub fn pin_input<P: Pin>(pin: P) -> PinInput {
    rcc::set_gpio_enabled(pin.port(), true);
    gpio::pin_input((pin.port(), pin.index()))
}

pub fn pin_altfn<P: Pin>(pin: P) -> PinAltFn {
    rcc::set_gpio_enabled(pin.port(), true);
    gpio::pin_altfn((pin.port(), pin.index()))
}

pub fn pin_analog<P: Pin>(pin: P) ->PinAnalog {
    rcc::set_gpio_enabled(pin.port(), true);
    gpio::pin_analog((pin.port(), pin.index()))
}
