pub use mcu::pin::*;

pub const BTN0: Pa12 = PA12;

pub fn init() {
    BTN0.port().gate_enable();
    BTN0.mode_input().pull_up();
}
