pub use mcu::pin::*;

pub const LED0: Pc0 = PC0;
pub const LED1: Pc2 = PC2;
pub const LED2: Pc3 = PC3;

pub fn init() {
    LED0.port().gate_enable();
    LED0.mode_output();

    LED1.port().gate_enable();
    LED1.mode_output();

    LED2.port().gate_enable();
    LED2.mode_output();
}
