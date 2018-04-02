#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::dcmi::*;

periph!( DCMI, Dcmi, DCMI_PERIPH, DcmiPeriph, DCMI_OWNED, DCMI_REF_COUNT, 0x50050000, 0x00, 0x13);

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("AHB2RSTR"), field: Some("DCMIRST"), description: None }
impl ::bobbin_common::gate::GateRst for Dcmi {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.ahb2rstr().dcmirst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2rstr(|r| r.set_dcmirst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("AHB2ENR"), field: Some("DCMIEN"), description: None }
impl ::bobbin_common::gate::GateEn for Dcmi {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.ahb2enr().dcmien() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2enr(|r| r.set_dcmien(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("AHB2LPENR"), field: Some("DCMILPEN"), description: None }
impl ::bobbin_common::gate::GateSleepEn for Dcmi {
    #[inline]
    fn gate_sleep_en(&self) -> bits::U1 { ::rcc::RCC.ahb2lpenr().dcmilpen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2lpenr(|r| r.set_dcmilpen(value));
        self
    }
}

