#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::crc::*;

periph!( CRC, Crc, CRC_PERIPH, CrcPeriph, 0x40032000, 0x00, 0x06);

// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC6"), field: Some("CRC"), description: None }
impl ::bobbin_common::gate::GateEn for Crc {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::sim::SIM.scgc6().crc() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc6(|r| r.set_crc(value));
        self
    }
}

