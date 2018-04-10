#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::wwdg::*;

periph!( WWDG, Wwdg, WWDG_PERIPH, WwdgPeriph, WWDG_OWNED, WWDG_REF_COUNT, 0x40002c00, 0x00, 0x06);

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB1RSTR"), field: Some("WWDGRST"), description: None }
impl ::bobbin_common::gate::GateRst for Wwdg {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.apb1rstr().wwdgrst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1rstr(|r| r.set_wwdgrst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB1ENR"), field: Some("WWDGEN"), description: None }
impl ::bobbin_common::gate::GateEn for Wwdg {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.apb1enr().wwdgen() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1enr(|r| r.set_wwdgen(value));
        self
    }
}

