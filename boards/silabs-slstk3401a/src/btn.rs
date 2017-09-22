
use hal::cmu::*;
use hal::gpio::*;

// BTN0 = PF6
// BTN1 = PF7

pub const BTN0: Pf6 = PF6;
pub const BTN1: Pf7 = PF7;

pub fn init() {
    // Enable GPIO
    CMU.with_hfbusclken0(|r| r.set_gpio(true));
    BTN0.mode_input();
    BTN1.mode_input();
}