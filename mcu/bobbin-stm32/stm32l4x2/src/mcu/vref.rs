#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::vref::*;

periph!( VREFBUF, Vrefbuf, VREFBUF_PERIPH, VrefPeriph, 0x40010030, 0x00, 0x31);

