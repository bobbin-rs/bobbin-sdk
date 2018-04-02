#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::ltdc::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( LTDC, Ltdc, LTDC_PERIPH, LtdcPeriph, LTDC_OWNED, LTDC_REF_COUNT, 0x40016800, 0x00, 0x0d);


