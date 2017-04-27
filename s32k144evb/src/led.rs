use chip::port::PORTD;
use chip::pcc::PCC;
use hal::port;

// LED RED = PTD0

pub fn led0() -> port::PinOutput {
    unsafe {
        PCC.with_portd(|r| r.set_cgc(1));
    }
    port::pin(PORTD, 0).into_output()
}