use bobbin_common::bits::*;
pub use ::chip::systick::*;
pub use ::chip::exc::{Handler, EXC_SYSTICK};

pub enum ClockSource {
    External = 0,
    Internal = 1,
}

pub fn set_handler(handler: Option<Handler>) {
    EXC_SYSTICK.set_handler(handler);
}

pub fn count_flag() -> bool {
    SYSTICK.csr().countflag() != 0
}

pub fn clock_source() -> ClockSource {
    match SYSTICK.csr().clksource() {
        U1::B0 => ClockSource::External,
        U1::B1 => ClockSource::Internal,
    }
}

pub fn tick_interrupt() -> bool {
    SYSTICK.csr().tickint() != 0
}

pub fn set_tick_interrupt(value: bool) {
    let value = if value { 1 } else { 0 };
    SYSTICK.with_csr(|r| r.set_tickint(value));
}

pub fn enabled() -> bool {
    SYSTICK.csr().enable() != 0
}

pub fn set_enabled(value: bool) {
    let value = if value { 1 } else { 0 };
    SYSTICK.with_csr(|r| r.set_enable(value));
}

pub fn reload_value() -> u32 {
    SYSTICK.rvr().reload().into()
}

pub fn set_reload_value(value: u32) {
    SYSTICK.set_rvr(Rvr(0).set_reload(value));
}

pub fn current_value() -> u32 {
    SYSTICK.cvr().current().into()
}

pub fn set_current_value(value: u32) {
    SYSTICK.set_cvr(Cvr(0).set_current(value));
}

pub fn no_reference_clock() -> bool {
    SYSTICK.calib().noref() != 0
}

pub fn skew() -> bool {
    SYSTICK.calib().skew() != 0
}

pub fn ten_ms() -> u32 {
    SYSTICK.calib().tenms().into()
}