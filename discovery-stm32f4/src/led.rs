use chip::gpio::GPIOD;
use hal::rcc;
use hal::gpio;

// LD3 = PD13 (Orange)
// LD4 = PD12 (Green)
// LD5 = PD14 (Red)
// LD6 = PD15 (Blue)

pub fn led0() -> gpio::PinOutput {
    rcc::set_gpio_enabled(GPIOD, true);
    gpio::pin(GPIOD, 13).into_output()
}