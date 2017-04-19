use chip::port::PORTB;
use hal::sim;
use hal::port;

// LED RED = PB22

pub fn led0() -> port::PinOutput {
    sim::set_port_enabled(PORTB, true);
    port::pin(PORTB, 22).into_output()
}