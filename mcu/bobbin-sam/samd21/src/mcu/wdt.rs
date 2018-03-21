#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ::hal::wdt::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( WDT, Wdt, WDT_PERIPH, WdtPeriph, 0x40001000, 0x00, 0x04);


// Gate { name: None, gate_type: Some("EN"), periph: Some("PM"), register: Some("APBAMASK"), field: Some("WDT"), description: None }
impl ::bobbin_common::gate::GateEn for Wdt {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::pm::PM.apbamask().wdt() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::pm::PM.with_apbamask(|r| r.set_wdt(value));
        self
    }
}

