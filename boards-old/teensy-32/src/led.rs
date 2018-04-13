use hal::gpio::*;
pub use common::digital::DigitalOutput;

pub const LED0: Pc5 = PC5; // Red

pub fn init() {
    LED0.port_pin().port().sim_enable();
    LED0.port_pin().set_mux_gpio();
    LED0.set_dir_output().set_output(true);
    
}