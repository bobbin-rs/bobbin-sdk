#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::dfsdm::*;

periph!( DFSDM, Dfsdm, DFSDM_PERIPH, DfsdmPeriph, 0x40016000, 0x39);

