#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::gpio::*;

periph!( GPIOA, Gpioa, GPIOA_PERIPH, GpioPeriph, GPIOA_OWNED, GPIOA_REF_COUNT, 0x48000000, 0x00, 0x1d);
periph!( GPIOB, Gpiob, GPIOB_PERIPH, GpioPeriph, GPIOB_OWNED, GPIOB_REF_COUNT, 0x48000400, 0x01, 0x1e);
periph!( GPIOC, Gpioc, GPIOC_PERIPH, GpioPeriph, GPIOC_OWNED, GPIOC_REF_COUNT, 0x48000800, 0x02, 0x1f);
periph!( GPIOD, Gpiod, GPIOD_PERIPH, GpioPeriph, GPIOD_OWNED, GPIOD_REF_COUNT, 0x48000c00, 0x03, 0x20);
periph!( GPIOE, Gpioe, GPIOE_PERIPH, GpioPeriph, GPIOE_OWNED, GPIOE_REF_COUNT, 0x48001000, 0x04, 0x21);
periph!( GPIOF, Gpiof, GPIOF_PERIPH, GpioPeriph, GPIOF_OWNED, GPIOF_REF_COUNT, 0x48001400, 0x05, 0x22);
periph!( GPIOG, Gpiog, GPIOG_PERIPH, GpioPeriph, GPIOG_OWNED, GPIOG_REF_COUNT, 0x48001800, 0x06, 0x23);
periph!( GPIOH, Gpioh, GPIOH_PERIPH, GpioPeriph, GPIOH_OWNED, GPIOH_REF_COUNT, 0x48001c00, 0x07, 0x24);

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

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("AHB2RSTR"), field: Some("GPIODRST"), description: None }
impl ::bobbin_common::gate::GateRst for Gpiod {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.ahb2rstr().gpiodrst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2rstr(|r| r.set_gpiodrst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("AHB2ENR"), field: Some("GPIODEN"), description: None }
impl ::bobbin_common::gate::GateEn for Gpiod {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.ahb2enr().gpioden() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2enr(|r| r.set_gpioden(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("AHB2SMENR"), field: Some("GPIODSMEN"), description: None }
impl ::bobbin_common::gate::GateSleepEn for Gpiod {
    #[inline]
    fn gate_sleep_en(&self) -> bits::U1 { ::rcc::RCC.ahb2smenr().gpiodsmen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2smenr(|r| r.set_gpiodsmen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("AHB2RSTR"), field: Some("GPIOERST"), description: None }
impl ::bobbin_common::gate::GateRst for Gpioe {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.ahb2rstr().gpioerst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2rstr(|r| r.set_gpioerst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("AHB2ENR"), field: Some("GPIOEEN"), description: None }
impl ::bobbin_common::gate::GateEn for Gpioe {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.ahb2enr().gpioeen() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2enr(|r| r.set_gpioeen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("AHB2SMENR"), field: Some("GPIOESMEN"), description: None }
impl ::bobbin_common::gate::GateSleepEn for Gpioe {
    #[inline]
    fn gate_sleep_en(&self) -> bits::U1 { ::rcc::RCC.ahb2smenr().gpioesmen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2smenr(|r| r.set_gpioesmen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("AHB2RSTR"), field: Some("GPIOHRST"), description: None }
impl ::bobbin_common::gate::GateRst for Gpioh {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.ahb2rstr().gpiohrst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2rstr(|r| r.set_gpiohrst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("AHB2ENR"), field: Some("GPIOHEN"), description: None }
impl ::bobbin_common::gate::GateEn for Gpioh {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.ahb2enr().gpiohen() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2enr(|r| r.set_gpiohen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("AHB2SMENR"), field: Some("GPIOHSMEN"), description: None }
impl ::bobbin_common::gate::GateSleepEn for Gpioh {
    #[inline]
    fn gate_sleep_en(&self) -> bits::U1 { ::rcc::RCC.ahb2smenr().gpiohsmen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2smenr(|r| r.set_gpiohsmen(value));
        self
    }
}

