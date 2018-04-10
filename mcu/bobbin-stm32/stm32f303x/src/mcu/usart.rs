#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::usart::*;

periph!( USART1, Usart1, USART1_PERIPH, UsartPeriph, USART1_OWNED, USART1_REF_COUNT, 0x40013800, 0x00, 0x11);
periph!( USART2, Usart2, USART2_PERIPH, UsartPeriph, USART2_OWNED, USART2_REF_COUNT, 0x40004400, 0x01, 0x12);
periph!( USART3, Usart3, USART3_PERIPH, UsartPeriph, USART3_OWNED, USART3_REF_COUNT, 0x40004800, 0x02, 0x13);
periph!( UART4, Uart4, UART4_PERIPH, UsartPeriph, UART4_OWNED, UART4_REF_COUNT, 0x40004c00, 0x03, 0x14);
periph!( UART5, Uart5, UART5_PERIPH, UsartPeriph, UART5_OWNED, UART5_REF_COUNT, 0x40005000, 0x04, 0x15);

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

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB1RSTR"), field: Some("USART2RST"), description: None }
impl ::bobbin_common::gate::GateRst for Usart2 {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.apb1rstr().usart2rst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1rstr(|r| r.set_usart2rst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB1ENR"), field: Some("USART2EN"), description: None }
impl ::bobbin_common::gate::GateEn for Usart2 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.apb1enr().usart2en() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1enr(|r| r.set_usart2en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB1RSTR"), field: Some("USART3RST"), description: None }
impl ::bobbin_common::gate::GateRst for Usart3 {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.apb1rstr().usart3rst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1rstr(|r| r.set_usart3rst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB1ENR"), field: Some("USART3EN"), description: None }
impl ::bobbin_common::gate::GateEn for Usart3 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.apb1enr().usart3en() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1enr(|r| r.set_usart3en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB1RSTR"), field: Some("UART4RST"), description: None }
impl ::bobbin_common::gate::GateRst for Uart4 {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.apb1rstr().uart4rst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1rstr(|r| r.set_uart4rst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB1ENR"), field: Some("UART4EN"), description: None }
impl ::bobbin_common::gate::GateEn for Uart4 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.apb1enr().uart4en() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1enr(|r| r.set_uart4en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB1RSTR"), field: Some("UART5RST"), description: None }
impl ::bobbin_common::gate::GateRst for Uart5 {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.apb1rstr().uart5rst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1rstr(|r| r.set_uart5rst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB1ENR"), field: Some("UART5EN"), description: None }
impl ::bobbin_common::gate::GateEn for Uart5 {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.apb1enr().uart5en() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1enr(|r| r.set_uart5en(value));
        self
    }
}

