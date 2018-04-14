use hal::port::*;
use hal::gpio::*;

pub use common::hal::digital::DigitalOutput;

pub const LED0: Pb22 = PB22; // Red
pub const LED1: Pb21 = PB21; // Blue
pub const LED2: Pe26 = PE26; // Green

pub fn init() {
    LED0.port_pin().port().sim_enable();
    LED0.port_pin().set_mux_gpio();
    LED0.set_dir_output().set_output(true);    

    LED1.port_pin().port().sim_enable();
    LED1.port_pin().set_mux_gpio();
    LED1.set_dir_output().set_output(true);    

    LED2.port_pin().port().sim_enable();
    LED2.port_pin().set_mux_gpio();
    LED2.set_dir_output().set_output(true);    

}
