pub use kinetis_common::i2c::*;

::bobbin_mcu::periph!( I2C0, I2c0, I2C0_PERIPH, I2cPeriph, I2C0_OWNED, I2C0_REF_COUNT, 0x40066000, 0x00, 0x12);
::bobbin_mcu::periph!( I2C1, I2c1, I2C1_PERIPH, I2cPeriph, I2C1_OWNED, I2C1_REF_COUNT, 0x40067000, 0x01, 0x13);

// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC4"), field: Some("I2C0"), description: None }
impl ::bobbin_mcu::gate::GateEn for I2c0 {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::sim::SIM.scgc4().i2c0() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc4(|r| r.set_i2c0(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC4"), field: Some("I2C1"), description: None }
impl ::bobbin_mcu::gate::GateEn for I2c1 {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::sim::SIM.scgc4().i2c1() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc4(|r| r.set_i2c1(value));
        self
    }
}

