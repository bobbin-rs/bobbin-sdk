use chip::port::PORTE;
use hal::sim;
use hal::port;

// Red => (GpioE, 29),
// Green => (GpioE, 31),
// Blue => (GpioD, 5),

pub fn led0() -> port::PinOutput {
    sim::set_port_enabled(PORTE, true);
    port::pin(PORTE, 29).into_output()
}