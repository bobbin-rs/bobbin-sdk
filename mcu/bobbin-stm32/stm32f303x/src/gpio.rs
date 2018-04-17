pub use ::stm32_common::gpio::*;

::bobbin_mcu::periph!( GPIOA, Gpioa, GPIOA_PERIPH, GpioPeriph, GPIOA_OWNED, GPIOA_REF_COUNT, 0x48000000, 0x00, 0x09);
::bobbin_mcu::periph!( GPIOB, Gpiob, GPIOB_PERIPH, GpioPeriph, GPIOB_OWNED, GPIOB_REF_COUNT, 0x48000400, 0x01, 0x0a);
::bobbin_mcu::periph!( GPIOC, Gpioc, GPIOC_PERIPH, GpioPeriph, GPIOC_OWNED, GPIOC_REF_COUNT, 0x48000800, 0x02, 0x0b);
::bobbin_mcu::periph!( GPIOD, Gpiod, GPIOD_PERIPH, GpioPeriph, GPIOD_OWNED, GPIOD_REF_COUNT, 0x48000c00, 0x03, 0x0c);
::bobbin_mcu::periph!( GPIOE, Gpioe, GPIOE_PERIPH, GpioPeriph, GPIOE_OWNED, GPIOE_REF_COUNT, 0x48001000, 0x04, 0x0d);
::bobbin_mcu::periph!( GPIOF, Gpiof, GPIOF_PERIPH, GpioPeriph, GPIOF_OWNED, GPIOF_REF_COUNT, 0x48001400, 0x05, 0x0e);
::bobbin_mcu::periph!( GPIOG, Gpiog, GPIOG_PERIPH, GpioPeriph, GPIOG_OWNED, GPIOG_REF_COUNT, 0x48001800, 0x06, 0x0f);
::bobbin_mcu::periph!( GPIOH, Gpioh, GPIOH_PERIPH, GpioPeriph, GPIOH_OWNED, GPIOH_REF_COUNT, 0x48001c00, 0x07, 0x10);

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("AHBRSTR"), field: Some("IOPARST"), description: None }
impl ::bobbin_mcu::gate::GateRst for Gpioa {
    #[inline]
    fn gate_rst(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahbrstr().ioparst() }
    #[inline]
    fn set_gate_rst<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahbrstr(|r| r.set_ioparst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("AHBENR"), field: Some("IOPAEN"), description: None }
impl ::bobbin_mcu::gate::GateEn for Gpioa {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahbenr().iopaen() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahbenr(|r| r.set_iopaen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("AHBRSTR"), field: Some("IOPBRST"), description: None }
impl ::bobbin_mcu::gate::GateRst for Gpiob {
    #[inline]
    fn gate_rst(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahbrstr().iopbrst() }
    #[inline]
    fn set_gate_rst<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahbrstr(|r| r.set_iopbrst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("AHBENR"), field: Some("IOPBEN"), description: None }
impl ::bobbin_mcu::gate::GateEn for Gpiob {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahbenr().iopben() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahbenr(|r| r.set_iopben(value));
        self
    }
}

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("AHBRSTR"), field: Some("IOPCRST"), description: None }
impl ::bobbin_mcu::gate::GateRst for Gpioc {
    #[inline]
    fn gate_rst(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahbrstr().iopcrst() }
    #[inline]
    fn set_gate_rst<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahbrstr(|r| r.set_iopcrst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("AHBENR"), field: Some("IOPCEN"), description: None }
impl ::bobbin_mcu::gate::GateEn for Gpioc {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahbenr().iopcen() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahbenr(|r| r.set_iopcen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("AHBRSTR"), field: Some("IOPDRST"), description: None }
impl ::bobbin_mcu::gate::GateRst for Gpiod {
    #[inline]
    fn gate_rst(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahbrstr().iopdrst() }
    #[inline]
    fn set_gate_rst<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahbrstr(|r| r.set_iopdrst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("AHBENR"), field: Some("IOPDEN"), description: None }
impl ::bobbin_mcu::gate::GateEn for Gpiod {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahbenr().iopden() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahbenr(|r| r.set_iopden(value));
        self
    }
}

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("AHBRSTR"), field: Some("IOPERST"), description: None }
impl ::bobbin_mcu::gate::GateRst for Gpioe {
    #[inline]
    fn gate_rst(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahbrstr().ioperst() }
    #[inline]
    fn set_gate_rst<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahbrstr(|r| r.set_ioperst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("AHBENR"), field: Some("IOPEEN"), description: None }
impl ::bobbin_mcu::gate::GateEn for Gpioe {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahbenr().iopeen() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahbenr(|r| r.set_iopeen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("AHBRSTR"), field: Some("IOPFRST"), description: None }
impl ::bobbin_mcu::gate::GateRst for Gpiof {
    #[inline]
    fn gate_rst(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahbrstr().iopfrst() }
    #[inline]
    fn set_gate_rst<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahbrstr(|r| r.set_iopfrst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("AHBENR"), field: Some("IOPFEN"), description: None }
impl ::bobbin_mcu::gate::GateEn for Gpiof {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahbenr().iopfen() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahbenr(|r| r.set_iopfen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("AHBRSTR"), field: Some("IOPGRST"), description: None }
impl ::bobbin_mcu::gate::GateRst for Gpiog {
    #[inline]
    fn gate_rst(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahbrstr().iopgrst() }
    #[inline]
    fn set_gate_rst<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahbrstr(|r| r.set_iopgrst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("AHBENR"), field: Some("IOPGEN"), description: None }
impl ::bobbin_mcu::gate::GateEn for Gpiog {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahbenr().iopgen() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahbenr(|r| r.set_iopgen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("AHBRSTR"), field: Some("IOPHRST"), description: None }
impl ::bobbin_mcu::gate::GateRst for Gpioh {
    #[inline]
    fn gate_rst(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahbrstr().iophrst() }
    #[inline]
    fn set_gate_rst<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahbrstr(|r| r.set_iophrst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("AHBENR"), field: Some("IOPHEN"), description: None }
impl ::bobbin_mcu::gate::GateEn for Gpioh {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahbenr().iophen() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahbenr(|r| r.set_iophen(value));
        self
    }
}

