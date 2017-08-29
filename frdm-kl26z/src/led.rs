use hal::gpio::*;
use hal::port::*;

pub use common::digital::DigitalOutput;


pub const LED0: Pe29 = PE29;
pub const LED0_PT: Pte29 = PTE29;

pub const LED1: Pe31 = PE31;
pub const LED1_PT: Pte31 = PTE31;

pub const LED2: Pd5 = PD5;
pub const LED2_PT: Ptd5 = PTD5;

pub fn init() {
    LED0_PT.port().sim_enable();
    LED0_PT.set_mux_gpio();
    LED0.set_dir_output().set_output(true);
    
    LED1_PT.port().sim_enable();
    LED1_PT.set_mux_gpio();
    LED1.set_dir_output().set_output(true);

    LED2_PT.port().sim_enable();
    LED2_PT.set_mux_gpio();
    LED2.set_dir_output().set_output(true);    
}
