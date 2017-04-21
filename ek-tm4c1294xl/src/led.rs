use chip::gpio::GPIOF;
use hal::sysctl;
use hal::gpio;

// Led0 => (GpioF, 0),
// Led1 => (GpioF, 4),
// Led2 => (GpioN, 0),
// Led3 => (GpioN, 1),


pub fn led0() -> gpio::PinOutput {
    sysctl::set_gpio_enabled(GPIOF, true);
    gpio::pin(GPIOF, 0).into_output()
}