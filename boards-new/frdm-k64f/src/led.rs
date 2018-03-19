use mcu::gpio::*;
use mcu::pin::*;
pub use common::digital::DigitalOutput;

pub const LED0: Pb22 = PB22;

pub fn init() {
    // Connect LED0 to GPIO
    PTB22.port().gate_enable();
    PTB22.connect_to(LED0);
    
    LED0.set_dir_output().set_output(true);
}
