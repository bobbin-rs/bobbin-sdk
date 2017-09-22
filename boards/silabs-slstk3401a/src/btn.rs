
use hal::cmu::*;
use hal::gpio::*;

// BTN0 = PF6
// BTN1 = PF7

pub const BTN0: Pf6 = PF6;
pub const BTN1: Pf7 = PF7;

pub fn init() {
    // Enable GPIO
    CMU.with_hfbusclken0(|r| r.set_gpio(true));
    // Set PF6 Mode = Input
    GPIOF.with_model(|r| r.set_mode(6, 0b0001));
    // Set PF7 Mode = Input
    GPIOF.with_model(|r| r.set_mode(7, 0b0001));

}