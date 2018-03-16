use mcu::pin::*;

pub const LED0: Pa17 = PA17;

pub fn init() {
    LED0.port().gate_enable();
    LED0.set_mode_output();
}