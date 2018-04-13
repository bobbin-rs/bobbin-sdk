use mcu::pin::*;

pub const LED0: Pb3 = PB3;

pub fn init() {
    LED0.port().gate_enable();
    LED0.mode_output();
}