use hal::gpio;
use pin;

pub fn init() {    
}

pub fn btn0() -> gpio::PinInput { 
    pin::pj0().into_input().with_pullup(true)
}
pub fn btn1() -> gpio::PinInput { 
    pin::pj1().into_input().with_pullup(true)
}