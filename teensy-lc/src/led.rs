use hal::port::*;
use hal::gpio::{GpioExt, DigitalOutput};

pub const LED0: Ptc5 = PTC5;

pub fn init() {
    LED0.port.sim_enable();
    LED0.gpio_pin().set_dir_output().set_output(true);
    LED0.set_mux_gpio();

}