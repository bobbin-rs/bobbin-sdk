#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::dmamux::*;

periph!( DMAMUX, Dmamux, DMAMUX_PERIPH, DmamuxPeriph, 0x40021000, 0x08);

// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC6"), field: Some("DMAMUX"), description: None }
impl ::bobbin_common::gate::GateEn for Dmamux {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::sim::SIM.scgc6().dmamux() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc6(|r| r.set_dmamux(value));
        self
    }
}

