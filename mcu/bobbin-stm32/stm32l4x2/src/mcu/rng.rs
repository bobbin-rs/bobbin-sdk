#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::rng::*;

periph!( RNG, Rng, RNG_PERIPH, RngPeriph, 0x50060800, 0x00, 0x12);

