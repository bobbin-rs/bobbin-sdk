use ::chip::pm::*;
use ::chip::port::*;
use ::chip::sercom::*;
use ::chip::rtc::*;
use ::chip::tc::*;
use ::chip::tcc::*;
use ::chip::adc::*;

pub fn set_port_enabled(_port: Port, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        PM.with_apbbmask(|r| r.set_port(value))
    }
}

pub fn set_sercom_enabled(sercom: Sercom, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        match sercom {
            SERCOM0 => PM.with_apbcmask(|r| r.set_sercom0(value)),
            SERCOM1 => PM.with_apbcmask(|r| r.set_sercom1(value)),
            SERCOM2 => PM.with_apbcmask(|r| r.set_sercom2(value)),
            SERCOM3 => PM.with_apbcmask(|r| r.set_sercom3(value)),
            SERCOM4 => PM.with_apbcmask(|r| r.set_sercom4(value)),
            SERCOM5 => PM.with_apbcmask(|r| r.set_sercom5(value)),
            _ => unimplemented!(),
        }        
    }
}

pub fn set_rtc_enabled(_rtc: Rtc, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        PM.with_apbamask(|r| r.set_rtc(value))
    }
}

pub fn set_tc_enabled(tc: Tc, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        match tc {
            TC3 => PM.with_apbcmask(|r| r.set_tc3(value)),
            TC4 => PM.with_apbcmask(|r| r.set_tc4(value)),
            TC5 => PM.with_apbcmask(|r| r.set_tc5(value)),
            _ => unimplemented!(),
        }
    }
}

pub fn set_tcc_enabled(tc: Tcc, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        match tc {
            TCC0 => PM.with_apbcmask(|r| r.set_tcc0(value)),
            TCC1 => PM.with_apbcmask(|r| r.set_tcc1(value)),
            TCC2 => PM.with_apbcmask(|r| r.set_tcc2(value)),
            _ => unimplemented!(),
        }
    }
}

pub fn set_adc_enabled(adc: Adc, value: bool) {
    let value = if value { 1 }  else { 0 };
    unsafe {
        match adc {
            ADC => PM.with_apbcmask(|r| r.set_adc(value)),
            _ => unimplemented!(),
        }
    }
}