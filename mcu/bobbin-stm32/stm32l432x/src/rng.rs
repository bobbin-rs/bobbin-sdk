pub use ::stm32_common::rng::*;

::bobbin_mcu::periph!( RNG, Rng, RNG_PERIPH, RngPeriph, RNG_OWNED, RNG_REF_COUNT, 0x50060800, 0x00, 0x06);

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("AHB2RSTR"), field: Some("RNGRST"), description: None }
impl ::bobbin_mcu::gate::GateRst for Rng {
    #[inline]
    fn gate_rst(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahb2rstr().rngrst() }
    #[inline]
    fn set_gate_rst<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2rstr(|r| r.set_rngrst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("AHB2ENR"), field: Some("RNGEN"), description: None }
impl ::bobbin_mcu::gate::GateEn for Rng {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahb2enr().rngen() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2enr(|r| r.set_rngen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("AHB2SMENR"), field: Some("RNGSMEN"), description: None }
impl ::bobbin_mcu::gate::GateSleepEn for Rng {
    #[inline]
    fn gate_sleep_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.ahb2smenr().rngsmen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb2smenr(|r| r.set_rngsmen(value));
        self
    }
}

