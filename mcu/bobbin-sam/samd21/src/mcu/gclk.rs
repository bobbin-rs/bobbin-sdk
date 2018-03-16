#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::gclk::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( GCLK, Gclk, GCLK_PERIPH, GclkPeriph, 0x40000c00, 0x00);


// Gate { name: None, gate_type: Some("EN"), periph: Some("PM"), register: Some("APBAMASK"), field: Some("GCLK"), description: None }
impl ::bobbin_common::gate::GateEn for Gclk {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::pm::PM.apbamask().gclk() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::pm::PM.with_apbamask(|r| r.set_gclk(value));
        self
    }
}

