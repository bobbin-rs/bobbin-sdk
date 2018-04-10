#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::rcc::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( RCC, Rcc, RCC_PERIPH, RccPeriph, RCC_OWNED, RCC_REF_COUNT, 0x40021000, 0x00, 0x00);


