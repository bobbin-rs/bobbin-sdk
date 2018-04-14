use hal::gpio::*;

pub use common::hal::digital::DigitalOutput;


pub const LED0: Pe29 = PE29;
pub const LED1: Pe31 = PE31;
pub const LED2: Pd5 = PD5;

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
