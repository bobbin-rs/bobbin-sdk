#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::pit::*;

periph!( PIT, Pit, PIT_PERIPH, PitPeriph, PIT_OWNED, PIT_REF_COUNT, 0x40037000, 0x00, 0x09);

