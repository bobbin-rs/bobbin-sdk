pub use mcu::pin::*;

pub const BTN0: Pc13 = PC13;

pub fn init() {
    BTN0.port().gate_enable();
    BTN0.mode_input().pull_down();
}
