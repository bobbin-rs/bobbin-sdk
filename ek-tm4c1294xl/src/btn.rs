use chip::gpio::GPIOC;
use hal::sysctl;
use hal::gpio;

// BTN0 = PC13
pub fn btn0() -> gpio::PinInput { 
    sysctl::set_gpio_enabled(GPIOC, true);
    gpio::pin(GPIOC, 13).into_input().with_pullup(true)
}