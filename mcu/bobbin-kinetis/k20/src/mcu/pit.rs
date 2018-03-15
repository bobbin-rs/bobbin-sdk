#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::pit::*;

periph!( PIT, Pit, PIT_PERIPH, PitPeriph, 0x40037000, 0x09);

