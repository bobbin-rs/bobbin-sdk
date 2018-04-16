#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::lpi2c::*;

periph!( LPI2C0, Lpi2c0, LPI2C0_PERIPH, Lpi2cPeriph, LPI2C0_OWNED, LPI2C0_REF_COUNT, 0x40066000, 0x00, 0x20);

