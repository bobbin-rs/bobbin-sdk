#[allow(unused_imports)] use ::bobbin_common::*;
#[allow(unused_imports)] pub use ::bobbin_common::gate::GateEn;
pub use ext::ethernet_ptp::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( ETHERNET_PTP, EthernetPtp, ETHERNET_PTP_PERIPH, EthernetPtpPeriph, 0x40028700, 0x00, 0x07);


// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("AHB1ENR"), field: Some("ETHMACPTPEN"), description: None }
impl ::bobbin_common::gate::GateEn for EthernetPtp {
    #[inline]
    fn gate_en(&self) -> bits::U1 { ::rcc::RCC.ahb1enr().ethmacptpen() }
    #[inline]
    fn set_gate_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb1enr(|r| r.set_ethmacptpen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("AHB1LPENR"), field: Some("ETHMACPTPLPEN"), description: None }
impl ::bobbin_common::gate::GateSleepEn for EthernetPtp {
    #[inline]
    fn gate_sleep_en(&self) -> bits::U1 { ::rcc::RCC.ahb1lpenr().ethmacptplpen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_ahb1lpenr(|r| r.set_ethmacptplpen(value));
        self
    }
}

