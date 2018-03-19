use mcu::pin::*;
pub use common::digital::DigitalOutput;

pub const LED0: Pb22 = PB22;

pub fn init() {
    // Set PTB22 to GPIO
    PTB22.port().gate_enable();
    PTB22.set_mux_gpio();
    // Set PTB22 to Output
    PB22.set_dir_output();
}
