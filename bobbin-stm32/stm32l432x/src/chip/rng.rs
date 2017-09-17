#[allow(unused_imports)] use bobbin_common::*;

pub use stm32_common::chip::rng::*;

periph!( RNG, Rng, _RNG, RngPeriph, 0x50060800);




