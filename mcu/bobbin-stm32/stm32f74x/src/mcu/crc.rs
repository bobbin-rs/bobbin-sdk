#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::crc::*;

periph!( CRC, Crc, CRC_PERIPH, CrcPeriph, 0x40023000, 0x00, 0x1a);

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("AHB1RSTR"), field: Some("CRCRST"), description: None }
impl ::bobbin_common::gate::GateRst for Crc {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.ahb1rstr().crcrst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb1rstr(|r| r.set_crcrst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("AHB1ENR"), field: Some("CRCEN"), description: None }
impl ::bobbin_common::gate::GateEn for Crc {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.ahb1enr().crcen() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb1enr(|r| r.set_crcen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("AHB1LPENR"), field: Some("CRCLPEN"), description: None }
impl ::bobbin_common::gate::GateSleepEn for Crc {
    #[inline]
    fn gate_sleep_en(&self) -> bits::U1 { ::rcc::RCC.ahb1lpenr().crclpen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb1lpenr(|r| r.set_crclpen(value));
        self
    }
}

