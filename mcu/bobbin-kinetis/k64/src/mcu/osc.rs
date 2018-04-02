#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::osc::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( OSC, Osc, OSC_PERIPH, OscPeriph, OSC_OWNED, OSC_REF_COUNT, 0x40065000, 0x00, 0x03);


