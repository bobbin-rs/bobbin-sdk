#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::osc::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( OSC, Osc, OSC_PERIPH, OscPeriph, 0x40065000, 0x00, 0x03);


