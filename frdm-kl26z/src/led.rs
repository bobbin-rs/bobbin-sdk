use hal::port::*;
use hal::gpio::{GpioExt, DigitalOutput};

pub const LED0: Pte29 = PTE29;
pub const LED1: Pte31 = PTE31;
pub const LED2: Ptd5 = PTD5;

pub fn init() {
    LED0.port().sim_enable();
    LED0.gpio_pin().set_dir_output().set_output(true);
    LED0.set_mux_gpio();

    LED1.port().sim_enable();
    LED1.gpio_pin().set_dir_output().set_output(true);
    LED1.set_mux_gpio();

    LED2.port().sim_enable();
    LED2.gpio_pin().set_dir_output().set_output(true);
    LED2.set_mux_gpio();
    
}


// use chip::port::PORTE;
// use hal::sim;
// use hal::port;

// // Red => (GpioE, 29),
// // Green => (GpioE, 31),
// // Blue => (GpioD, 5),

// pub fn led0() -> port::PinOutput {
//     sim::set_port_enabled(PORTE, true);
//     port::pin(PORTE, 29).into_output()
// }