#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::dmamux::*;

periph!( DMAMUX, Dmamux, DMAMUX_PERIPH, DmamuxPeriph, DMAMUX_OWNED, DMAMUX_REF_COUNT, 0x40021000, 0x00, 0x04);

