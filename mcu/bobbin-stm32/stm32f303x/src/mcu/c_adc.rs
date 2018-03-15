#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::c_adc::*;

periph!( C_ADC12, CAdc12, C_ADC12_PERIPH, CAdcPeriph, 0x50000300, 0x20);
periph!( C_ADC34, CAdc34, C_ADC34_PERIPH, CAdcPeriph, 0x50000700, 0x21);

