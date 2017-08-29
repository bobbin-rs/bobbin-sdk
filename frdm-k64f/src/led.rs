use hal::port::*;
use hal::gpio::*;

pub use common::digital::DigitalOutput;

pub const LED0: Pb22 = PB22; // Red
pub const LED0_PT: Ptb22 = PTB22; // Red

pub const LED1: Pb21 = PB21; // Blue
pub const LED1_PT: Ptb21 = PTB21; // Blue

pub const LED2: Pe26 = PE26; // Green
pub const LED2_PT: Pte26 = PTE26; // Green


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
