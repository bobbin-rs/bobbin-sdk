#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::rng::*;

periph!( RNG, Rng, RNG_PERIPH, RngPeriph, 0x50060800, 0x12);

