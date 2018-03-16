#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::enet::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( ENET, Enet, ENET_PERIPH, EnetPeriph, 0x400c0000, 0x05);


// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC2"), field: Some("ENET"), description: None }
impl ::bobbin_common::gate::GateEn for Enet {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::sim::SIM.scgc2().enet() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc2(|r| r.set_enet(value));
        self
    }
}

