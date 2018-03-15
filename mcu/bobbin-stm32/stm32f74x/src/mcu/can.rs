#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::can::*;

periph!( CAN1, Can1, CAN1_PERIPH, CanPeriph, 0x40006400, 0x3a);
periph!( CAN2, Can2, CAN2_PERIPH, CanPeriph, 0x40006800, 0x3b);

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB1RSTR"), field: Some("CAN1RST"), description: None }
impl ::bobbin_common::gate::GateRst for Can1 {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.apb1rstr().can1rst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1rstr(|r| r.set_can1rst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB1ENR"), field: Some("CAN1EN"), description: None }
impl ::bobbin_common::gate::GateEn for Can1 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.apb1enr().can1en() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1enr(|r| r.set_can1en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("APB1LPENR"), field: Some("CAN1LPEN"), description: None }
impl ::bobbin_common::gate::GateSleepEn for Can1 {
    #[inline]
    fn gate_sleep_en(&self) -> bits::U1 { ::rcc::RCC.apb1lpenr().can1lpen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1lpenr(|r| r.set_can1lpen(value));
        self
    }
}

