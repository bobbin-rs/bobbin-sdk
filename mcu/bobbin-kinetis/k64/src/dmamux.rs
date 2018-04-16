pub use kinetis_common::dmamux::*;

::bobbin_mcu::periph!( DMAMUX, Dmamux, DMAMUX_PERIPH, DmamuxPeriph, DMAMUX_OWNED, DMAMUX_REF_COUNT, 0x40021000, 0x00, 0x09);

// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC6"), field: Some("DMAMUX"), description: None }
impl ::bobbin_mcu::gate::GateEn for Dmamux {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::sim::SIM.scgc6().dmamux() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc6(|r| r.set_dmamux(value));
        self
    }
}

