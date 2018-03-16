#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::pm::*;

#[allow(unused_imports)] use ::bobbin_common::*;


periph!( PM, Pm, PM_PERIPH, PmPeriph, 0x40000400, 0x02);

impl En for super::pm::Pm {
    #[inline] fn en(&self) -> bits::U1 { PM.apbamask().pm() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { PM.with_apbamask(|r| r.set_pm(value)); }
}

impl En for super::sysctrl::Sysctrl {
    #[inline] fn en(&self) -> bits::U1 { PM.apbamask().sysctrl() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { PM.with_apbamask(|r| r.set_sysctrl(value)); }
}

impl En for super::gclk::Gclk {
    #[inline] fn en(&self) -> bits::U1 { PM.apbamask().gclk() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { PM.with_apbamask(|r| r.set_gclk(value)); }
}

impl En for super::wdt::Wdt {
    #[inline] fn en(&self) -> bits::U1 { PM.apbamask().wdt() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { PM.with_apbamask(|r| r.set_wdt(value)); }
}

impl En for super::rtc::Rtc {
    #[inline] fn en(&self) -> bits::U1 { PM.apbamask().rtc() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { PM.with_apbamask(|r| r.set_rtc(value)); }
}

impl En for super::port::Porta {
    #[inline] fn en(&self) -> bits::U1 { PM.apbbmask().port() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { PM.with_apbbmask(|r| r.set_port(value)); }
}

impl En for super::port::Portb {
    #[inline] fn en(&self) -> bits::U1 { PM.apbbmask().port() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { PM.with_apbbmask(|r| r.set_port(value)); }
}

impl En for super::sercom::Sercom0 {
    #[inline] fn en(&self) -> bits::U1 { PM.apbcmask().sercom0() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { PM.with_apbcmask(|r| r.set_sercom0(value)); }
}

impl En for super::sercom::Sercom1 {
    #[inline] fn en(&self) -> bits::U1 { PM.apbcmask().sercom1() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { PM.with_apbcmask(|r| r.set_sercom1(value)); }
}

impl En for super::sercom::Sercom2 {
    #[inline] fn en(&self) -> bits::U1 { PM.apbcmask().sercom2() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { PM.with_apbcmask(|r| r.set_sercom2(value)); }
}

impl En for super::sercom::Sercom3 {
    #[inline] fn en(&self) -> bits::U1 { PM.apbcmask().sercom3() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { PM.with_apbcmask(|r| r.set_sercom3(value)); }
}

impl En for super::sercom::Sercom4 {
    #[inline] fn en(&self) -> bits::U1 { PM.apbcmask().sercom4() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { PM.with_apbcmask(|r| r.set_sercom4(value)); }
}

impl En for super::sercom::Sercom5 {
    #[inline] fn en(&self) -> bits::U1 { PM.apbcmask().sercom5() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { PM.with_apbcmask(|r| r.set_sercom5(value)); }
}

impl En for super::tcc::Tcc0 {
    #[inline] fn en(&self) -> bits::U1 { PM.apbcmask().tcc0() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { PM.with_apbcmask(|r| r.set_tcc0(value)); }
}

impl En for super::tcc::Tcc1 {
    #[inline] fn en(&self) -> bits::U1 { PM.apbcmask().tcc1() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { PM.with_apbcmask(|r| r.set_tcc1(value)); }
}

impl En for super::tcc::Tcc2 {
    #[inline] fn en(&self) -> bits::U1 { PM.apbcmask().tcc2() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { PM.with_apbcmask(|r| r.set_tcc2(value)); }
}

impl En for super::tc::Tc3 {
    #[inline] fn en(&self) -> bits::U1 { PM.apbcmask().tc3() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { PM.with_apbcmask(|r| r.set_tc3(value)); }
}

impl En for super::tc::Tc4 {
    #[inline] fn en(&self) -> bits::U1 { PM.apbcmask().tc4() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { PM.with_apbcmask(|r| r.set_tc4(value)); }
}

impl En for super::tc::Tc5 {
    #[inline] fn en(&self) -> bits::U1 { PM.apbcmask().tc5() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { PM.with_apbcmask(|r| r.set_tc5(value)); }
}

impl En for super::adc::Adc {
    #[inline] fn en(&self) -> bits::U1 { PM.apbcmask().adc() }
    #[inline] fn set_en<V: Into<bits::U1>>(&self, value: V) { PM.with_apbcmask(|r| r.set_adc(value)); }
}


