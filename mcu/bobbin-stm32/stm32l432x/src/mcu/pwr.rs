#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::pwr::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( PWR, Pwr, PWR_PERIPH, PwrPeriph, 0x40007000, 0x00, 0x01);


// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB1RSTR1"), field: Some("PWRRST"), description: None }
impl ::bobbin_common::gate::GateRst for Pwr {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.apb1rstr1().pwrrst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1rstr1(|r| r.set_pwrrst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB1ENR1"), field: Some("PWREN"), description: None }
impl ::bobbin_common::gate::GateEn for Pwr {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.apb1enr1().pwren() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1enr1(|r| r.set_pwren(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("APB1SMENR1"), field: Some("PWRSMEN"), description: None }
impl ::bobbin_common::gate::GateSleepEn for Pwr {
    #[inline]
    fn gate_sleep_en(&self) -> bits::U1 { ::rcc::RCC.apb1smenr1().pwrsmen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1smenr1(|r| r.set_pwrsmen(value));
        self
    }
}

