pub use ::stm32_common::wwdg::*;

::bobbin_mcu::periph!( WWDG, Wwdg, WWDG_PERIPH, WwdgPeriph, WWDG_OWNED, WWDG_REF_COUNT, 0x40002c00, 0x00, 0x0b);

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB1RSTR"), field: Some("WWDGRST"), description: None }
impl ::bobbin_mcu::gate::GateRst for Wwdg {
    #[inline]
    fn gate_rst(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb1rstr().wwdgrst() }
    #[inline]
    fn set_gate_rst<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1rstr(|r| r.set_wwdgrst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB1ENR"), field: Some("WWDGEN"), description: None }
impl ::bobbin_mcu::gate::GateEn for Wwdg {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb1enr().wwdgen() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1enr(|r| r.set_wwdgen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("APB1LPENR"), field: Some("WWDGLPEN"), description: None }
impl ::bobbin_mcu::gate::GateSleepEn for Wwdg {
    #[inline]
    fn gate_sleep_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb1lpenr().wwdglpen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1lpenr(|r| r.set_wwdglpen(value));
        self
    }
}

