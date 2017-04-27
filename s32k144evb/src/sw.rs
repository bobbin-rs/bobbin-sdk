use hal::port;
use pin;

// SW2 = PTC12
// SW3 = PTC13

pub fn sw2() -> port::PinInput { 
    pin::ptc12().into_input().with_pull(port::Pull::PullUp)
}

pub fn sw3() -> port::PinInput { 
    pin::ptc13().into_input().with_pull(port::Pull::PullUp)
}