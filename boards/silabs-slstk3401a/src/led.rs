
use hal::cmu::*;
use hal::gpio::*;

// LED0 = PF4
// LED1 = PF5
// BTN0 = PF6
// BTN1 = PF7

pub const LED0: Pf4 = PF4;
pub const LED1: Pf5 = PF5;

pub fn init() {
    // Enable GPIO
    CMU.with_hfbusclken0(|r| r.set_gpio(true));
    LED0.mode_output();
    LED1.mode_output();    
}