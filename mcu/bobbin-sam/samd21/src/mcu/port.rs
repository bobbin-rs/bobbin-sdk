#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::port::*;

periph!( PORTA, Porta, PORTA_PERIPH, PortPeriph, 0x41004400, 0x00, 0x0f);
periph!( PORTB, Portb, PORTB_PERIPH, PortPeriph, 0x41004480, 0x01, 0x10);

// Gate { name: None, gate_type: Some("EN"), periph: Some("PM"), register: Some("APBBMASK"), field: Some("PORT"), description: None }
impl ::bobbin_common::gate::GateEn for Porta {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::pm::PM.apbbmask().port() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::pm::PM.with_apbbmask(|r| r.set_port(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("PM"), register: Some("APBBMASK"), field: Some("PORT"), description: None }
impl ::bobbin_common::gate::GateEn for Portb {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::pm::PM.apbbmask().port() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::pm::PM.with_apbbmask(|r| r.set_port(value));
        self
    }
}

