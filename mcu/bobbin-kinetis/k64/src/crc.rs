pub use kinetis_common::crc::*;

::bobbin_mcu::periph!( CRC, Crc, CRC_PERIPH, CrcPeriph, CRC_OWNED, CRC_REF_COUNT, 0x40032000, 0x00, 0x05);

// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC6"), field: Some("CRC"), description: None }
impl ::bobbin_mcu::gate::GateEn for Crc {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::sim::SIM.scgc6().crc() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc6(|r| r.set_crc(value));
        self
    }
}

