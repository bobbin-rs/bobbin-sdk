#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::cryp::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( CRYP, Cryp, CRYP_PERIPH, CrypPeriph, 0x50060000, 0x00, 0x11);


// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("AHB2RSTR"), field: Some("CRYPRST"), description: None }
impl ::bobbin_common::gate::GateRst for Cryp {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.ahb2rstr().cryprst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2rstr(|r| r.set_cryprst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("AHB2ENR"), field: Some("CRYPEN"), description: None }
impl ::bobbin_common::gate::GateEn for Cryp {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.ahb2enr().crypen() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2enr(|r| r.set_crypen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("AHB2LPENR"), field: Some("CRYPLPEN"), description: None }
impl ::bobbin_common::gate::GateSleepEn for Cryp {
    #[inline]
    fn gate_sleep_en(&self) -> bits::U1 { ::rcc::RCC.ahb2lpenr().cryplpen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2lpenr(|r| r.set_cryplpen(value));
        self
    }
}

