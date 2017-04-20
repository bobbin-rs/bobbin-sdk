use chip::port::{PORTA, PORTC};
use hal::{sim, port};

// SW2 = PTC6
// SW3 = PTA4

pub fn sw2() -> port::PinInput { 
    sim::set_port_enabled(PORTC, true);
    port::pin(PORTC, 6).into_input().with_pull(port::Pull::PullUp)
}

pub fn sw3() -> port::PinInput { 
    sim::set_port_enabled(PORTA, true);
    port::pin(PORTA, 4).into_input().with_pull(port::Pull::PullUp)
}