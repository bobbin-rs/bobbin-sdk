use chip::gpio::*;
use hal::gpio;

// LD3 = PD13 (Orange)
// LD4 = PD12 (Green)
// LD5 = PD14 (Red)
// LD6 = PD15 (Blue)

pub type Led = gpio::PinOutput;

pub fn init() {
    gpio::pin(PD13).into_output();
    gpio::pin(PD12).into_output();
    gpio::pin(PD14).into_output();
    gpio::pin(PD15).into_output();
}

pub fn led0() -> Led {
    gpio::pin_output(PD13)
}

pub fn led1() -> Led {
    gpio::pin_output(PD12)
}
pub fn led2() -> Led {
    gpio::pin_output(PD14)
}
pub fn led3() -> Led {
    gpio::pin_output(PD15)
}

// use chip::gpio::GPIOD;
// use hal::rcc;
// use hal::gpio;

// // LD3 = PD13 (Orange)
// // LD4 = PD12 (Green)
// // LD5 = PD14 (Red)
// // LD6 = PD15 (Blue)

// pub fn led0() -> gpio::PinOutput {
//     rcc::set_gpio_enabled(GPIOD, true);
//     gpio::pin(GPIOD, 13).into_output()
// }