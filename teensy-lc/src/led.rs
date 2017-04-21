use chip::port::PORTC;
use hal::sim;
use hal::port;

// Red => PTC5

pub fn led0() -> port::PinOutput {
    sim::set_port_enabled(PORTC, true);
    port::pin(PORTC, 5).into_output()
}