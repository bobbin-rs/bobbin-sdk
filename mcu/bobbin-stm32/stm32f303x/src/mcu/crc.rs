#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::crc::*;

periph!( CRC, Crc, CRC_PERIPH, CrcPeriph, CRC_OWNED, CRC_REF_COUNT, 0x40023000, 0x00, 0x07);

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("AHBENR"), field: Some("CRCEN"), description: None }
impl ::bobbin_common::gate::GateEn for Crc {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.ahbenr().crcen() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahbenr(|r| r.set_crcen(value));
        self
    }
}

