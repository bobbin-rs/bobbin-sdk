use ::chip::lptim::*;

pub enum Clock {
    Internal = 0,
    External = 1,
}

pub enum Prescaler {
    Div1 = 0b000,
    Div2 = 0b001,
    Div4 = 0b010,
    Div8 = 0b011,
    Div16 = 0b100,
    Div32 = 0b101,
    Div64 = 0b110,
    Div128 = 0b111,
}

pub fn set_autoreload_ie(mut lptim: Lptim, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        lptim.with_ier(|r| r.set_arrmie(value));
    }    
}

pub fn set_compare_ie(mut lptim: Lptim, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        lptim.with_ier(|r| r.set_cmpmie(value));
    }    
}

pub fn set_clock(mut lptim: Lptim, clock: Clock) {
    unsafe {
        lptim.with_cfgr(|r| r.set_cksel(clock as u32))
    }
}

pub fn set_prescaler(mut lptim: Lptim, presc: Prescaler) {
    unsafe {
        lptim.with_cfgr(|r| r.set_presc(presc as u32))
    }
}

pub fn set_enabled(mut lptim: Lptim, value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        lptim.with_cr(|r| r.set_enable(value));
    }
}

pub fn set_compare(mut lptim: Lptim, value: u16) {
    unsafe {
        lptim.set_cmp(Cmp(0).set_cmp(value as u32));
    }
}

pub fn set_autoreload(mut lptim: Lptim, value: u16) {
    unsafe {
        lptim.set_arr(Arr(0).set_arr(value as u32));
    }
}

pub fn start_single(mut lptim: Lptim) {
    unsafe {
        lptim.with_cr(|r| r.set_sngstrt(1))
    }
}

pub fn start_continuous(mut lptim: Lptim) {
    unsafe {
        lptim.with_cr(|r| r.set_cntstrt(1))
    }
}

pub fn counter(lptim: Lptim) -> u16 {
    unsafe {
        lptim.cnt().cnt() as u16
    }
}