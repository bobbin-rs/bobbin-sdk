pub use ::bobbin_common::mcu::*;
use ::bobbin_common::owned::*;

pub mod gclk;
pub mod nvmctrl;
pub mod pm;
pub mod sysctrl;
pub mod wdt;
pub mod rtc;
pub mod dmac;
pub mod adc;
pub mod dac;
pub mod tcc;
pub mod tc;
pub mod port;
pub mod sercom;
pub mod sig;
pub mod pin;
pub mod irq;

#[derive(Debug, Default)]
pub struct Samd21 {}

impl Mcu for Samd21 {
    fn id(&self) -> &'static str { "SAMD21" }
}

impl Samd21 {
    pub fn gclk(&self) -> Option<Owned<gclk::Gclk>> { gclk::Gclk::acquire() }
    pub fn nvmctrl(&self) -> Option<Owned<nvmctrl::Nvmctrl>> { nvmctrl::Nvmctrl::acquire() }
    pub fn pm(&self) -> Option<Owned<pm::Pm>> { pm::Pm::acquire() }
    pub fn sysctrl(&self) -> Option<Owned<sysctrl::Sysctrl>> { sysctrl::Sysctrl::acquire() }
    pub fn wdt(&self) -> Option<Owned<wdt::Wdt>> { wdt::Wdt::acquire() }
    pub fn rtc(&self) -> Option<Owned<rtc::Rtc>> { rtc::Rtc::acquire() }
    pub fn dmac(&self) -> Option<Owned<dmac::Dmac>> { dmac::Dmac::acquire() }
        pub fn dmac_ch0(&self) -> Option<Owned<dmac::DmacCh0>> { dmac::DmacCh0::acquire() }
        pub fn dmac_ch1(&self) -> Option<Owned<dmac::DmacCh1>> { dmac::DmacCh1::acquire() }
        pub fn dmac_ch2(&self) -> Option<Owned<dmac::DmacCh2>> { dmac::DmacCh2::acquire() }
        pub fn dmac_ch3(&self) -> Option<Owned<dmac::DmacCh3>> { dmac::DmacCh3::acquire() }
        pub fn dmac_ch4(&self) -> Option<Owned<dmac::DmacCh4>> { dmac::DmacCh4::acquire() }
        pub fn dmac_ch5(&self) -> Option<Owned<dmac::DmacCh5>> { dmac::DmacCh5::acquire() }
        pub fn dmac_ch6(&self) -> Option<Owned<dmac::DmacCh6>> { dmac::DmacCh6::acquire() }
        pub fn dmac_ch7(&self) -> Option<Owned<dmac::DmacCh7>> { dmac::DmacCh7::acquire() }
        pub fn dmac_ch8(&self) -> Option<Owned<dmac::DmacCh8>> { dmac::DmacCh8::acquire() }
        pub fn dmac_ch9(&self) -> Option<Owned<dmac::DmacCh9>> { dmac::DmacCh9::acquire() }
        pub fn dmac_ch10(&self) -> Option<Owned<dmac::DmacCh10>> { dmac::DmacCh10::acquire() }
        pub fn dmac_ch11(&self) -> Option<Owned<dmac::DmacCh11>> { dmac::DmacCh11::acquire() }
    pub fn adc(&self) -> Option<Owned<adc::Adc>> { adc::Adc::acquire() }
        pub fn adc_ch0(&self) -> Option<Owned<adc::AdcCh0>> { adc::AdcCh0::acquire() }
        pub fn adc_ch1(&self) -> Option<Owned<adc::AdcCh1>> { adc::AdcCh1::acquire() }
        pub fn adc_ch2(&self) -> Option<Owned<adc::AdcCh2>> { adc::AdcCh2::acquire() }
        pub fn adc_ch3(&self) -> Option<Owned<adc::AdcCh3>> { adc::AdcCh3::acquire() }
        pub fn adc_ch4(&self) -> Option<Owned<adc::AdcCh4>> { adc::AdcCh4::acquire() }
        pub fn adc_ch5(&self) -> Option<Owned<adc::AdcCh5>> { adc::AdcCh5::acquire() }
        pub fn adc_ch6(&self) -> Option<Owned<adc::AdcCh6>> { adc::AdcCh6::acquire() }
        pub fn adc_ch7(&self) -> Option<Owned<adc::AdcCh7>> { adc::AdcCh7::acquire() }
        pub fn adc_ch8(&self) -> Option<Owned<adc::AdcCh8>> { adc::AdcCh8::acquire() }
        pub fn adc_ch9(&self) -> Option<Owned<adc::AdcCh9>> { adc::AdcCh9::acquire() }
        pub fn adc_ch10(&self) -> Option<Owned<adc::AdcCh10>> { adc::AdcCh10::acquire() }
        pub fn adc_ch11(&self) -> Option<Owned<adc::AdcCh11>> { adc::AdcCh11::acquire() }
        pub fn adc_ch12(&self) -> Option<Owned<adc::AdcCh12>> { adc::AdcCh12::acquire() }
        pub fn adc_ch13(&self) -> Option<Owned<adc::AdcCh13>> { adc::AdcCh13::acquire() }
        pub fn adc_ch14(&self) -> Option<Owned<adc::AdcCh14>> { adc::AdcCh14::acquire() }
        pub fn adc_ch15(&self) -> Option<Owned<adc::AdcCh15>> { adc::AdcCh15::acquire() }
        pub fn adc_ch16(&self) -> Option<Owned<adc::AdcCh16>> { adc::AdcCh16::acquire() }
        pub fn adc_ch17(&self) -> Option<Owned<adc::AdcCh17>> { adc::AdcCh17::acquire() }
        pub fn adc_ch18(&self) -> Option<Owned<adc::AdcCh18>> { adc::AdcCh18::acquire() }
        pub fn adc_ch19(&self) -> Option<Owned<adc::AdcCh19>> { adc::AdcCh19::acquire() }
        pub fn adc_temp(&self) -> Option<Owned<adc::AdcTemp>> { adc::AdcTemp::acquire() }
        pub fn adc_bandgap(&self) -> Option<Owned<adc::AdcBandgap>> { adc::AdcBandgap::acquire() }
        pub fn adc_scaled_core(&self) -> Option<Owned<adc::AdcScaledCore>> { adc::AdcScaledCore::acquire() }
        pub fn adc_scaled_io(&self) -> Option<Owned<adc::AdcScaledIo>> { adc::AdcScaledIo::acquire() }
        pub fn adc_dac(&self) -> Option<Owned<adc::AdcDac>> { adc::AdcDac::acquire() }
    pub fn dac(&self) -> Option<Owned<dac::Dac>> { dac::Dac::acquire() }
    pub fn tcc0(&self) -> Option<Owned<tcc::Tcc0>> { tcc::Tcc0::acquire() }
        pub fn tcc0_ch0(&self) -> Option<Owned<tcc::Tcc0Ch0>> { tcc::Tcc0Ch0::acquire() }
        pub fn tcc0_ch1(&self) -> Option<Owned<tcc::Tcc0Ch1>> { tcc::Tcc0Ch1::acquire() }
        pub fn tcc0_ch2(&self) -> Option<Owned<tcc::Tcc0Ch2>> { tcc::Tcc0Ch2::acquire() }
        pub fn tcc0_ch3(&self) -> Option<Owned<tcc::Tcc0Ch3>> { tcc::Tcc0Ch3::acquire() }
    pub fn tcc1(&self) -> Option<Owned<tcc::Tcc1>> { tcc::Tcc1::acquire() }
        pub fn tcc1_ch0(&self) -> Option<Owned<tcc::Tcc1Ch0>> { tcc::Tcc1Ch0::acquire() }
        pub fn tcc1_ch1(&self) -> Option<Owned<tcc::Tcc1Ch1>> { tcc::Tcc1Ch1::acquire() }
    pub fn tcc2(&self) -> Option<Owned<tcc::Tcc2>> { tcc::Tcc2::acquire() }
        pub fn tcc2_ch0(&self) -> Option<Owned<tcc::Tcc2Ch0>> { tcc::Tcc2Ch0::acquire() }
        pub fn tcc2_ch1(&self) -> Option<Owned<tcc::Tcc2Ch1>> { tcc::Tcc2Ch1::acquire() }
    pub fn tc3(&self) -> Option<Owned<tc::Tc3>> { tc::Tc3::acquire() }
        pub fn tc3_ch0(&self) -> Option<Owned<tc::Tc3Ch0>> { tc::Tc3Ch0::acquire() }
        pub fn tc3_ch1(&self) -> Option<Owned<tc::Tc3Ch1>> { tc::Tc3Ch1::acquire() }
    pub fn tc4(&self) -> Option<Owned<tc::Tc4>> { tc::Tc4::acquire() }
        pub fn tc4_ch0(&self) -> Option<Owned<tc::Tc4Ch0>> { tc::Tc4Ch0::acquire() }
        pub fn tc4_ch1(&self) -> Option<Owned<tc::Tc4Ch1>> { tc::Tc4Ch1::acquire() }
    pub fn tc5(&self) -> Option<Owned<tc::Tc5>> { tc::Tc5::acquire() }
        pub fn tc5_ch0(&self) -> Option<Owned<tc::Tc5Ch0>> { tc::Tc5Ch0::acquire() }
        pub fn tc5_ch1(&self) -> Option<Owned<tc::Tc5Ch1>> { tc::Tc5Ch1::acquire() }
    pub fn porta(&self) -> Option<Owned<port::Porta>> { port::Porta::acquire() }
        pub fn pa00(&self) -> Option<Owned<pin::Pa00>> { pin::Pa00::acquire() }
        pub fn pa01(&self) -> Option<Owned<pin::Pa01>> { pin::Pa01::acquire() }
        pub fn pa02(&self) -> Option<Owned<pin::Pa02>> { pin::Pa02::acquire() }
        pub fn pa03(&self) -> Option<Owned<pin::Pa03>> { pin::Pa03::acquire() }
        pub fn pa04(&self) -> Option<Owned<pin::Pa04>> { pin::Pa04::acquire() }
        pub fn pa05(&self) -> Option<Owned<pin::Pa05>> { pin::Pa05::acquire() }
        pub fn pa06(&self) -> Option<Owned<pin::Pa06>> { pin::Pa06::acquire() }
        pub fn pa07(&self) -> Option<Owned<pin::Pa07>> { pin::Pa07::acquire() }
        pub fn pa08(&self) -> Option<Owned<pin::Pa08>> { pin::Pa08::acquire() }
        pub fn pa09(&self) -> Option<Owned<pin::Pa09>> { pin::Pa09::acquire() }
        pub fn pa10(&self) -> Option<Owned<pin::Pa10>> { pin::Pa10::acquire() }
        pub fn pa11(&self) -> Option<Owned<pin::Pa11>> { pin::Pa11::acquire() }
        pub fn pa12(&self) -> Option<Owned<pin::Pa12>> { pin::Pa12::acquire() }
        pub fn pa13(&self) -> Option<Owned<pin::Pa13>> { pin::Pa13::acquire() }
        pub fn pa14(&self) -> Option<Owned<pin::Pa14>> { pin::Pa14::acquire() }
        pub fn pa15(&self) -> Option<Owned<pin::Pa15>> { pin::Pa15::acquire() }
        pub fn pa16(&self) -> Option<Owned<pin::Pa16>> { pin::Pa16::acquire() }
        pub fn pa17(&self) -> Option<Owned<pin::Pa17>> { pin::Pa17::acquire() }
        pub fn pa18(&self) -> Option<Owned<pin::Pa18>> { pin::Pa18::acquire() }
        pub fn pa19(&self) -> Option<Owned<pin::Pa19>> { pin::Pa19::acquire() }
        pub fn pa20(&self) -> Option<Owned<pin::Pa20>> { pin::Pa20::acquire() }
        pub fn pa21(&self) -> Option<Owned<pin::Pa21>> { pin::Pa21::acquire() }
        pub fn pa22(&self) -> Option<Owned<pin::Pa22>> { pin::Pa22::acquire() }
        pub fn pa23(&self) -> Option<Owned<pin::Pa23>> { pin::Pa23::acquire() }
        pub fn pa24(&self) -> Option<Owned<pin::Pa24>> { pin::Pa24::acquire() }
        pub fn pa25(&self) -> Option<Owned<pin::Pa25>> { pin::Pa25::acquire() }
        pub fn pa27(&self) -> Option<Owned<pin::Pa27>> { pin::Pa27::acquire() }
        pub fn pa28(&self) -> Option<Owned<pin::Pa28>> { pin::Pa28::acquire() }
        pub fn pa30(&self) -> Option<Owned<pin::Pa30>> { pin::Pa30::acquire() }
        pub fn pa31(&self) -> Option<Owned<pin::Pa31>> { pin::Pa31::acquire() }
    pub fn portb(&self) -> Option<Owned<port::Portb>> { port::Portb::acquire() }
        pub fn pb00(&self) -> Option<Owned<pin::Pb00>> { pin::Pb00::acquire() }
        pub fn pb01(&self) -> Option<Owned<pin::Pb01>> { pin::Pb01::acquire() }
        pub fn pb02(&self) -> Option<Owned<pin::Pb02>> { pin::Pb02::acquire() }
        pub fn pb03(&self) -> Option<Owned<pin::Pb03>> { pin::Pb03::acquire() }
        pub fn pb04(&self) -> Option<Owned<pin::Pb04>> { pin::Pb04::acquire() }
        pub fn pb05(&self) -> Option<Owned<pin::Pb05>> { pin::Pb05::acquire() }
        pub fn pb06(&self) -> Option<Owned<pin::Pb06>> { pin::Pb06::acquire() }
        pub fn pb07(&self) -> Option<Owned<pin::Pb07>> { pin::Pb07::acquire() }
        pub fn pb08(&self) -> Option<Owned<pin::Pb08>> { pin::Pb08::acquire() }
        pub fn pb09(&self) -> Option<Owned<pin::Pb09>> { pin::Pb09::acquire() }
        pub fn pb10(&self) -> Option<Owned<pin::Pb10>> { pin::Pb10::acquire() }
        pub fn pb11(&self) -> Option<Owned<pin::Pb11>> { pin::Pb11::acquire() }
        pub fn pb12(&self) -> Option<Owned<pin::Pb12>> { pin::Pb12::acquire() }
        pub fn pb13(&self) -> Option<Owned<pin::Pb13>> { pin::Pb13::acquire() }
        pub fn pb14(&self) -> Option<Owned<pin::Pb14>> { pin::Pb14::acquire() }
        pub fn pb15(&self) -> Option<Owned<pin::Pb15>> { pin::Pb15::acquire() }
        pub fn pb16(&self) -> Option<Owned<pin::Pb16>> { pin::Pb16::acquire() }
        pub fn pb17(&self) -> Option<Owned<pin::Pb17>> { pin::Pb17::acquire() }
        pub fn pb22(&self) -> Option<Owned<pin::Pb22>> { pin::Pb22::acquire() }
        pub fn pb23(&self) -> Option<Owned<pin::Pb23>> { pin::Pb23::acquire() }
        pub fn pb30(&self) -> Option<Owned<pin::Pb30>> { pin::Pb30::acquire() }
        pub fn pb31(&self) -> Option<Owned<pin::Pb31>> { pin::Pb31::acquire() }
    pub fn sercom0(&self) -> Option<Owned<sercom::Sercom0>> { sercom::Sercom0::acquire() }
    pub fn sercom1(&self) -> Option<Owned<sercom::Sercom1>> { sercom::Sercom1::acquire() }
    pub fn sercom2(&self) -> Option<Owned<sercom::Sercom2>> { sercom::Sercom2::acquire() }
    pub fn sercom3(&self) -> Option<Owned<sercom::Sercom3>> { sercom::Sercom3::acquire() }
    pub fn sercom4(&self) -> Option<Owned<sercom::Sercom4>> { sercom::Sercom4::acquire() }
    pub fn sercom5(&self) -> Option<Owned<sercom::Sercom5>> { sercom::Sercom5::acquire() }
}

impl Get<gclk::Gclk> for Samd21 {
    fn get(&self) -> gclk::Gclk { gclk::GCLK }
}

impl GetPeriph<gclk::GclkPeriph> for Samd21 {
    fn get_periph(&self) -> gclk::GclkPeriph { gclk::GCLK_PERIPH }
}

impl GetPeriphInstance<gclk::GclkPeriph> for Samd21 {
    fn get_periph_instance(&self, index: usize) -> Option<gclk::GclkPeriph> {
        match index { 
            0 => Some(gclk::GCLK_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<nvmctrl::Nvmctrl> for Samd21 {
    fn get(&self) -> nvmctrl::Nvmctrl { nvmctrl::NVMCTRL }
}

impl GetPeriph<nvmctrl::NvmctrlPeriph> for Samd21 {
    fn get_periph(&self) -> nvmctrl::NvmctrlPeriph { nvmctrl::NVMCTRL_PERIPH }
}

impl GetPeriphInstance<nvmctrl::NvmctrlPeriph> for Samd21 {
    fn get_periph_instance(&self, index: usize) -> Option<nvmctrl::NvmctrlPeriph> {
        match index { 
            0 => Some(nvmctrl::NVMCTRL_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<pm::Pm> for Samd21 {
    fn get(&self) -> pm::Pm { pm::PM }
}

impl GetPeriph<pm::PmPeriph> for Samd21 {
    fn get_periph(&self) -> pm::PmPeriph { pm::PM_PERIPH }
}

impl GetPeriphInstance<pm::PmPeriph> for Samd21 {
    fn get_periph_instance(&self, index: usize) -> Option<pm::PmPeriph> {
        match index { 
            0 => Some(pm::PM_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<sysctrl::Sysctrl> for Samd21 {
    fn get(&self) -> sysctrl::Sysctrl { sysctrl::SYSCTRL }
}

impl GetPeriph<sysctrl::SysctrlPeriph> for Samd21 {
    fn get_periph(&self) -> sysctrl::SysctrlPeriph { sysctrl::SYSCTRL_PERIPH }
}

impl GetPeriphInstance<sysctrl::SysctrlPeriph> for Samd21 {
    fn get_periph_instance(&self, index: usize) -> Option<sysctrl::SysctrlPeriph> {
        match index { 
            0 => Some(sysctrl::SYSCTRL_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<wdt::Wdt> for Samd21 {
    fn get(&self) -> wdt::Wdt { wdt::WDT }
}

impl GetPeriph<wdt::WdtPeriph> for Samd21 {
    fn get_periph(&self) -> wdt::WdtPeriph { wdt::WDT_PERIPH }
}

impl GetPeriphInstance<wdt::WdtPeriph> for Samd21 {
    fn get_periph_instance(&self, index: usize) -> Option<wdt::WdtPeriph> {
        match index { 
            0 => Some(wdt::WDT_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<rtc::Rtc> for Samd21 {
    fn get(&self) -> rtc::Rtc { rtc::RTC }
}

impl GetPeriph<rtc::RtcPeriph> for Samd21 {
    fn get_periph(&self) -> rtc::RtcPeriph { rtc::RTC_PERIPH }
}

impl GetPeriphInstance<rtc::RtcPeriph> for Samd21 {
    fn get_periph_instance(&self, index: usize) -> Option<rtc::RtcPeriph> {
        match index { 
            0 => Some(rtc::RTC_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<dmac::Dmac> for Samd21 {
    fn get(&self) -> dmac::Dmac { dmac::DMAC }
}

impl GetPeriph<dmac::DmacPeriph> for Samd21 {
    fn get_periph(&self) -> dmac::DmacPeriph { dmac::DMAC_PERIPH }
}

impl GetPeriphInstance<dmac::DmacPeriph> for Samd21 {
    fn get_periph_instance(&self, index: usize) -> Option<dmac::DmacPeriph> {
        match index {
            0 => Some(dmac::DMAC_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<adc::Adc> for Samd21 {
    fn get(&self) -> adc::Adc { adc::ADC }
}

impl GetPeriph<adc::AdcPeriph> for Samd21 {
    fn get_periph(&self) -> adc::AdcPeriph { adc::ADC_PERIPH }
}

impl GetPeriphInstance<adc::AdcPeriph> for Samd21 {
    fn get_periph_instance(&self, index: usize) -> Option<adc::AdcPeriph> {
        match index {
            0 => Some(adc::ADC_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<dac::Dac> for Samd21 {
    fn get(&self) -> dac::Dac { dac::DAC }
}

impl GetPeriph<dac::DacPeriph> for Samd21 {
    fn get_periph(&self) -> dac::DacPeriph { dac::DAC_PERIPH }
}

impl GetPeriphInstance<dac::DacPeriph> for Samd21 {
    fn get_periph_instance(&self, index: usize) -> Option<dac::DacPeriph> {
        match index {
            0 => Some(dac::DAC_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 1 }
}

impl Get<tcc::Tcc0> for Samd21 {
    fn get(&self) -> tcc::Tcc0 { tcc::TCC0 }
}

impl Get<tcc::Tcc1> for Samd21 {
    fn get(&self) -> tcc::Tcc1 { tcc::TCC1 }
}

impl Get<tcc::Tcc2> for Samd21 {
    fn get(&self) -> tcc::Tcc2 { tcc::TCC2 }
}

impl GetPeriphInstance<tcc::TccPeriph> for Samd21 {
    fn get_periph_instance(&self, index: usize) -> Option<tcc::TccPeriph> {
        match index {
            0 => Some(tcc::TCC0_PERIPH),
            1 => Some(tcc::TCC1_PERIPH),
            2 => Some(tcc::TCC2_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 3 }
}

impl Get<tc::Tc3> for Samd21 {
    fn get(&self) -> tc::Tc3 { tc::TC3 }
}

impl Get<tc::Tc4> for Samd21 {
    fn get(&self) -> tc::Tc4 { tc::TC4 }
}

impl Get<tc::Tc5> for Samd21 {
    fn get(&self) -> tc::Tc5 { tc::TC5 }
}

impl GetPeriphInstance<tc::TcPeriph> for Samd21 {
    fn get_periph_instance(&self, index: usize) -> Option<tc::TcPeriph> {
        match index {
            0 => Some(tc::TC3_PERIPH),
            1 => Some(tc::TC4_PERIPH),
            2 => Some(tc::TC5_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 3 }
}

impl Get<port::Porta> for Samd21 {
    fn get(&self) -> port::Porta { port::PORTA }
}

impl Get<port::Portb> for Samd21 {
    fn get(&self) -> port::Portb { port::PORTB }
}

impl GetPeriphInstance<port::PortPeriph> for Samd21 {
    fn get_periph_instance(&self, index: usize) -> Option<port::PortPeriph> {
        match index {
            0 => Some(port::PORTA_PERIPH),
            1 => Some(port::PORTB_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 2 }
}

impl Get<sercom::Sercom0> for Samd21 {
    fn get(&self) -> sercom::Sercom0 { sercom::SERCOM0 }
}

impl Get<sercom::Sercom1> for Samd21 {
    fn get(&self) -> sercom::Sercom1 { sercom::SERCOM1 }
}

impl Get<sercom::Sercom2> for Samd21 {
    fn get(&self) -> sercom::Sercom2 { sercom::SERCOM2 }
}

impl Get<sercom::Sercom3> for Samd21 {
    fn get(&self) -> sercom::Sercom3 { sercom::SERCOM3 }
}

impl Get<sercom::Sercom4> for Samd21 {
    fn get(&self) -> sercom::Sercom4 { sercom::SERCOM4 }
}

impl Get<sercom::Sercom5> for Samd21 {
    fn get(&self) -> sercom::Sercom5 { sercom::SERCOM5 }
}

impl GetPeriphInstance<sercom::SercomPeriph> for Samd21 {
    fn get_periph_instance(&self, index: usize) -> Option<sercom::SercomPeriph> {
        match index {
            0 => Some(sercom::SERCOM0_PERIPH),
            1 => Some(sercom::SERCOM1_PERIPH),
            2 => Some(sercom::SERCOM2_PERIPH),
            3 => Some(sercom::SERCOM3_PERIPH),
            4 => Some(sercom::SERCOM4_PERIPH),
            5 => Some(sercom::SERCOM5_PERIPH),
            _ => None,
        }
    }
    fn get_periph_instance_count(&self) -> usize { 6 }
}

