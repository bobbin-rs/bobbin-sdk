#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::flexcan::*;

periph!( CAN0, Can0, CAN0_PERIPH, FlexcanPeriph, 0x40024000, 0x00, 0x1b);

// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC6"), field: Some("FLEXCAN0"), description: None }
impl ::bobbin_common::gate::GateEn for Can0 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::sim::SIM.scgc6().flexcan0() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc6(|r| r.set_flexcan0(value));
        self
    }
}

