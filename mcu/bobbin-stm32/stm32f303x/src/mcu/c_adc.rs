#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::c_adc::*;

periph!( C_ADC12, CAdc12, C_ADC12_PERIPH, CAdcPeriph, C_ADC12_OWNED, C_ADC12_REF_COUNT, 0x50000300, 0x00, 0x20);
periph!( C_ADC34, CAdc34, C_ADC34_PERIPH, CAdcPeriph, C_ADC34_OWNED, C_ADC34_REF_COUNT, 0x50000700, 0x01, 0x21);

