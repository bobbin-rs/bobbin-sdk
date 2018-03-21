#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::syscfg::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( SYSCFG, Syscfg, SYSCFG_PERIPH, SyscfgPeriph, 0x40010000, 0x00, 0x03);


// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB2RSTR"), field: Some("SYSCFGRST"), description: None }
impl ::bobbin_common::gate::GateRst for Syscfg {
    #[inline]
    fn gate_rst(&self) -> bits::U1 { ::rcc::RCC.apb2rstr().syscfgrst() }
    #[inline]
    fn set_gate_rst<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2rstr(|r| r.set_syscfgrst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB2ENR"), field: Some("SYSCFGEN"), description: None }
impl ::bobbin_common::gate::GateEn for Syscfg {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.apb2enr().syscfgen() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2enr(|r| r.set_syscfgen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("APB2SMENR"), field: Some("SYSCFGSMEN"), description: None }
impl ::bobbin_common::gate::GateSleepEn for Syscfg {
    #[inline]
    fn gate_sleep_en(&self) -> bits::U1 { ::rcc::RCC.apb2smenr().syscfgsmen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2smenr(|r| r.set_syscfgsmen(value));
        self
    }
}

