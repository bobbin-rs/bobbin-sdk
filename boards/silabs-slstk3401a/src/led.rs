
use hal::gpio::*;

// LED0 = PF4
// LED1 = PF5

pub const LED0: Pf4 = PF4;
pub const LED1: Pf5 = PF5;

pub fn init() {
    ::hal::gpio::init();
    LED0.mode_output();
    LED1.mode_output();    
}