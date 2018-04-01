#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::i2c::*;

periph!( I2C0, I2c0, I2C0_PERIPH, I2cPeriph, 0x40066000, 0x00, 0x12);
periph!( I2C1, I2c1, I2C1_PERIPH, I2cPeriph, 0x40067000, 0x01, 0x13);

// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC4"), field: Some("I2C0"), description: None }
impl ::bobbin_common::gate::GateEn for I2c0 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::sim::SIM.scgc4().i2c0() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc4(|r| r.set_i2c0(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC4"), field: Some("I2C1"), description: None }
impl ::bobbin_common::gate::GateEn for I2c1 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::sim::SIM.scgc4().i2c1() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc4(|r| r.set_i2c1(value));
        self
    }
}

