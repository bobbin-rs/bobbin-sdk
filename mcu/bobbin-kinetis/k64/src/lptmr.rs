pub use kinetis_common::lptmr::*;

::bobbin_mcu::periph!( LPTMR0, Lptmr0, LPTMR0_PERIPH, LptmrPeriph, LPTMR0_OWNED, LPTMR0_REF_COUNT, 0x40040000, 0x00, 0x0e);

// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC5"), field: Some("LPTMR"), description: None }
impl ::bobbin_mcu::gate::GateEn for Lptmr0 {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::sim::SIM.scgc5().lptmr() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc5(|r| r.set_lptmr(value));
        self
    }
}

