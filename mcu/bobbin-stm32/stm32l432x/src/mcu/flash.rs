#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::flash::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( FLASH, Flash, FLASH_PERIPH, FlashPeriph, FLASH_OWNED, FLASH_REF_COUNT, 0x40022000, 0x00, 0x00);


// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("AHB1RSTR"), field: Some("FLASHRST"), description: None }
impl ::bobbin_common::gate::GateRst for Flash {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.ahb1rstr().flashrst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb1rstr(|r| r.set_flashrst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("AHB1ENR"), field: Some("FLASHEN"), description: None }
impl ::bobbin_common::gate::GateEn for Flash {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.ahb1enr().flashen() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb1enr(|r| r.set_flashen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("AHB1SMENR"), field: Some("FLASHSMEN"), description: None }
impl ::bobbin_common::gate::GateSleepEn for Flash {
    #[inline]
    fn gate_sleep_en(&self) -> bits::U1 { ::rcc::RCC.ahb1smenr().flashsmen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb1smenr(|r| r.set_flashsmen(value));
        self
    }
}

