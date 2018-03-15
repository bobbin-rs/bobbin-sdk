#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::dmamux::*;

periph!( DMAMUX, Dmamux, DMAMUX_PERIPH, DmamuxPeriph, 0x40021000, 0x04);

