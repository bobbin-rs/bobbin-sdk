pub use ::bobbin_common::mcu::*;

pub mod gclk;
pub mod nvmctrl;
pub mod pm;
pub mod sysctrl;
pub mod wdt;
pub mod rtc;
pub mod dmac;
pub mod adc;
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
    pub fn gclk(&self) -> gclk::Gclk { gclk::GCLK }
    pub fn nvmctrl(&self) -> nvmctrl::Nvmctrl { nvmctrl::NVMCTRL }
    pub fn pm(&self) -> pm::Pm { pm::PM }
    pub fn sysctrl(&self) -> sysctrl::Sysctrl { sysctrl::SYSCTRL }
    pub fn wdt(&self) -> wdt::Wdt { wdt::WDT }
    pub fn rtc(&self) -> rtc::Rtc { rtc::RTC }
    pub fn dmac(&self) -> dmac::Dmac { dmac::DMAC }
    pub fn adc(&self) -> adc::Adc { adc::ADC }
    pub fn tcc0(&self) -> tcc::Tcc0 { tcc::TCC0 }
    pub fn tcc1(&self) -> tcc::Tcc1 { tcc::TCC1 }
    pub fn tcc2(&self) -> tcc::Tcc2 { tcc::TCC2 }
    pub fn tc3(&self) -> tc::Tc3 { tc::TC3 }
    pub fn tc4(&self) -> tc::Tc4 { tc::TC4 }
    pub fn tc5(&self) -> tc::Tc5 { tc::TC5 }
    pub fn porta(&self) -> port::Porta { port::PORTA }
    pub fn portb(&self) -> port::Portb { port::PORTB }
    pub fn sercom0(&self) -> sercom::Sercom0 { sercom::SERCOM0 }
    pub fn sercom1(&self) -> sercom::Sercom1 { sercom::SERCOM1 }
    pub fn sercom2(&self) -> sercom::Sercom2 { sercom::SERCOM2 }
    pub fn sercom3(&self) -> sercom::Sercom3 { sercom::SERCOM3 }
    pub fn sercom4(&self) -> sercom::Sercom4 { sercom::SERCOM4 }
    pub fn sercom5(&self) -> sercom::Sercom5 { sercom::SERCOM5 }
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

