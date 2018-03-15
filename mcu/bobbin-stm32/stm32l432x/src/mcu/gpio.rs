#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::gpio::*;

periph!( GPIOA, Gpioa, GPIOA_PERIPH, GpioPeriph, 0x48000000, 0x1c);
periph!( GPIOB, Gpiob, GPIOB_PERIPH, GpioPeriph, 0x48000400, 0x1d);
periph!( GPIOC, Gpioc, GPIOC_PERIPH, GpioPeriph, 0x48000800, 0x1e);
periph!( GPIOD, Gpiod, GPIOD_PERIPH, GpioPeriph, 0x48000c00, 0x1f);
periph!( GPIOE, Gpioe, GPIOE_PERIPH, GpioPeriph, 0x48001000, 0x20);
periph!( GPIOF, Gpiof, GPIOF_PERIPH, GpioPeriph, 0x48001400, 0x21);
periph!( GPIOG, Gpiog, GPIOG_PERIPH, GpioPeriph, 0x48001800, 0x22);
periph!( GPIOH, Gpioh, GPIOH_PERIPH, GpioPeriph, 0x48001c00, 0x23);

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("AHB2RSTR"), field: Some("GPIOARST"), description: None }
impl ::bobbin_common::gate::GateRst for Gpioa {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.ahb2rstr().gpioarst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2rstr(|r| r.set_gpioarst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("AHB2ENR"), field: Some("GPIOAEN"), description: None }
impl ::bobbin_common::gate::GateEn for Gpioa {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.ahb2enr().gpioaen() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2enr(|r| r.set_gpioaen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("AHB2SMENR"), field: Some("GPIOASMEN"), description: None }
impl ::bobbin_common::gate::GateSleepEn for Gpioa {
    #[inline]
    fn gate_sleep_en(&self) -> bits::U1 { ::rcc::RCC.ahb2smenr().gpioasmen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2smenr(|r| r.set_gpioasmen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("AHB2RSTR"), field: Some("GPIOBRST"), description: None }
impl ::bobbin_common::gate::GateRst for Gpiob {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.ahb2rstr().gpiobrst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2rstr(|r| r.set_gpiobrst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("AHB2ENR"), field: Some("GPIOBEN"), description: None }
impl ::bobbin_common::gate::GateEn for Gpiob {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.ahb2enr().gpioben() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2enr(|r| r.set_gpioben(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("AHB2SMENR"), field: Some("GPIOBSMEN"), description: None }
impl ::bobbin_common::gate::GateSleepEn for Gpiob {
    #[inline]
    fn gate_sleep_en(&self) -> bits::U1 { ::rcc::RCC.ahb2smenr().gpiobsmen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2smenr(|r| r.set_gpiobsmen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("AHB2RSTR"), field: Some("GPIOCRST"), description: None }
impl ::bobbin_common::gate::GateRst for Gpioc {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.ahb2rstr().gpiocrst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2rstr(|r| r.set_gpiocrst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("AHB2ENR"), field: Some("GPIOCEN"), description: None }
impl ::bobbin_common::gate::GateEn for Gpioc {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.ahb2enr().gpiocen() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2enr(|r| r.set_gpiocen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("AHB2SMENR"), field: Some("GPIOCSMEN"), description: None }
impl ::bobbin_common::gate::GateSleepEn for Gpioc {
    #[inline]
    fn gate_sleep_en(&self) -> bits::U1 { ::rcc::RCC.ahb2smenr().gpiocsmen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2smenr(|r| r.set_gpiocsmen(value));
        self
    }
}

