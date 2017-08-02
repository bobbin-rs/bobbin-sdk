use hal::port::*;
use hal::gpio::{GpioExt, DigitalOutput};

// LED BLUE = PTD0
// LED Red = PTD15
// LED Green = PTD16

pub const LED0: Ptd0 = PTD0;
pub const LED1: Ptd15 = PTD15;
pub const LED2: Ptd16 = PTD16;

pub fn init() {
    LED0.port().pcc_enable();
    LED0.gpio_pin().set_dir_output().set_output(true);
    LED0.set_mux_gpio();

    LED1.port().pcc_enable();
    LED1.gpio_pin().set_dir_output().set_output(true);
    LED1.set_mux_gpio();

    LED2.port().pcc_enable();
    LED2.gpio_pin().set_dir_output().set_output(true);
    LED2.set_mux_gpio();        
}


// pub fn led_blue() -> port::PinOutput {
//     pin::ptd0().into_output()
// }

// pub fn led_red() -> port::PinOutput {
//     pin::ptd15().into_output()
// }

// pub fn led_green() -> port::PinOutput {
//     pin::ptd16().into_output()
// }