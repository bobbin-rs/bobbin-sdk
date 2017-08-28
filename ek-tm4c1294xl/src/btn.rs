use hal::gpio::*;

pub const BTN0: Pj0 = PJ0;
pub const BTN1: Pj1 = PJ1;

pub fn init() {
    BTN0.port().sysctl_enable();
    BTN0.mode_input().pull_up();

    BTN1.port().sysctl_enable();
    BTN1.mode_input().pull_up();
}


// use hal::gpio;
// use pin;

// pub fn init() {    
// }

// pub fn btn0() -> gpio::PinInput { 
//     pin::pj0().into_input().with_pullup(true)
// }
// pub fn btn1() -> gpio::PinInput { 
//     pin::pj1().into_input().with_pullup(true)
// }