#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::usart::*;

periph!( USART1, Usart1, USART1_PERIPH, UsartPeriph, 0x40013800, 0x00, 0x24);
periph!( USART2, Usart2, USART2_PERIPH, UsartPeriph, 0x40004400, 0x01, 0x25);
periph!( USART3, Usart3, USART3_PERIPH, UsartPeriph, 0x40004800, 0x02, 0x26);
periph!( UART4, Uart4, UART4_PERIPH, UsartPeriph, 0x40004c00, 0x03, 0x27);
periph!( UART5, Uart5, UART5_PERIPH, UsartPeriph, 0x40005000, 0x04, 0x28);

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB2RSTR"), field: Some("USART1RST"), description: None }
impl ::bobbin_common::gate::GateRst for Usart1 {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.apb2rstr().usart1rst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2rstr(|r| r.set_usart1rst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB2ENR"), field: Some("USART1EN"), description: None }
impl ::bobbin_common::gate::GateEn for Usart1 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.apb2enr().usart1en() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2enr(|r| r.set_usart1en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("APB2SMENR"), field: Some("USART1SMEN"), description: None }
impl ::bobbin_common::gate::GateSleepEn for Usart1 {
    #[inline]
    fn gate_sleep_en(&self) -> bits::U1 { ::rcc::RCC.apb2smenr().usart1smen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2smenr(|r| r.set_usart1smen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB1RSTR1"), field: Some("USART2RST"), description: None }
impl ::bobbin_common::gate::GateRst for Usart2 {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.apb1rstr1().usart2rst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1rstr1(|r| r.set_usart2rst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB1ENR1"), field: Some("USART2EN"), description: None }
impl ::bobbin_common::gate::GateEn for Usart2 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.apb1enr1().usart2en() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1enr1(|r| r.set_usart2en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("APB1SMENR1"), field: Some("USART2SMEN"), description: None }
impl ::bobbin_common::gate::GateSleepEn for Usart2 {
    #[inline]
    fn gate_sleep_en(&self) -> bits::U1 { ::rcc::RCC.apb1smenr1().usart2smen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1smenr1(|r| r.set_usart2smen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB1RSTR1"), field: Some("USART2RST"), description: None }
impl ::bobbin_common::gate::GateRst for Usart3 {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.apb1rstr1().usart2rst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1rstr1(|r| r.set_usart2rst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB1ENR1"), field: Some("USART3EN"), description: None }
impl ::bobbin_common::gate::GateEn for Usart3 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.apb1enr1().usart3en() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1enr1(|r| r.set_usart3en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("APB1SMENR1"), field: Some("USART3SMEN"), description: None }
impl ::bobbin_common::gate::GateSleepEn for Usart3 {
    #[inline]
    fn gate_sleep_en(&self) -> bits::U1 { ::rcc::RCC.apb1smenr1().usart3smen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1smenr1(|r| r.set_usart3smen(value));
        self
    }
}

