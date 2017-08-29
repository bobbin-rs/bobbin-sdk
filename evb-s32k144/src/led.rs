use hal::gpio::*;
use hal::port::*;

pub use common::digital::DigitalOutput;

// pub const LED0: Pd0 = PD0;
pub const LED0: Pd0 = PD0;
pub const LEDPT0: Ptd0 = PTD0;
pub const LED1: Pd15 = PD15;
pub const LEDPT1: Ptd15 = PTD15;
pub const LED2: Pd16 = PD16;
pub const LEDPT2: Ptd16 = PTD16;

pub fn init() {
    PORTD.pcc_set_enabled(true);

    LEDPT0.set_mux_gpio();
    LED0.set_dir_output().set_output(true);
    
    LEDPT1.set_mux_gpio();
    LED1.set_dir_output().set_output(true);
    
    LEDPT2.set_mux_gpio();
    LED2.set_dir_output().set_output(true);
}