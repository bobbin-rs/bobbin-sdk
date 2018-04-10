#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::i2c::*;

periph!( I2C1, I2c1, I2C1_PERIPH, I2cPeriph, I2C1_OWNED, I2C1_REF_COUNT, 0x40005400, 0x00, 0x16);
periph!( I2C2, I2c2, I2C2_PERIPH, I2cPeriph, I2C2_OWNED, I2C2_REF_COUNT, 0x40005800, 0x01, 0x17);
periph!( I2C3, I2c3, I2C3_PERIPH, I2cPeriph, I2C3_OWNED, I2C3_REF_COUNT, 0x40007800, 0x02, 0x18);

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB1RSTR"), field: Some("I2C1RST"), description: None }
impl ::bobbin_common::gate::GateRst for I2c1 {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.apb1rstr().i2c1rst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1rstr(|r| r.set_i2c1rst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB1ENR"), field: Some("I2C1EN"), description: None }
impl ::bobbin_common::gate::GateEn for I2c1 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.apb1enr().i2c1en() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1enr(|r| r.set_i2c1en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB1RSTR"), field: Some("I2C2RST"), description: None }
impl ::bobbin_common::gate::GateRst for I2c2 {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.apb1rstr().i2c2rst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1rstr(|r| r.set_i2c2rst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB1ENR"), field: Some("I2C2EN"), description: None }
impl ::bobbin_common::gate::GateEn for I2c2 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.apb1enr().i2c2en() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1enr(|r| r.set_i2c2en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB1RSTR"), field: Some("I2C3RST"), description: None }
impl ::bobbin_common::gate::GateRst for I2c3 {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.apb1rstr().i2c3rst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1rstr(|r| r.set_i2c3rst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB1ENR"), field: Some("I2C3EN"), description: None }
impl ::bobbin_common::gate::GateEn for I2c3 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.apb1enr().i2c3en() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1enr(|r| r.set_i2c3en(value));
        self
    }
}

