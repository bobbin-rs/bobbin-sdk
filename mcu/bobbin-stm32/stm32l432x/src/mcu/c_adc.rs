#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::c_adc::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( C_ADC, CAdc, C_ADC_PERIPH, CAdcPeriph, 0x50040300, 0x00, 0x04);


