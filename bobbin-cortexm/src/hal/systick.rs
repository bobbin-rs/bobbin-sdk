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
    unsafe {
        SYSTICK.csr().countflag() != 0
    }
}

pub fn clock_source() -> ClockSource {
    unsafe {
        match SYSTICK.csr().clksource() {
            0 => ClockSource::External,
            1 => ClockSource::Internal,
            _ => unimplemented!(),
        }
    }
}

pub fn tick_interrupt() -> bool {
    unsafe {
        SYSTICK.csr().tickint() != 0
    }
}

pub fn set_tick_interrupt(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SYSTICK.with_csr(|r| r.set_tickint(value))
    }
}

pub fn enabled() -> bool {
    unsafe {
        SYSTICK.csr().enable() != 0
    }
}

pub fn set_enabled(value: bool) {
    let value = if value { 1 } else { 0 };
    unsafe {
        SYSTICK.with_csr(|r| r.set_enable(value))
    }
}

pub fn reload_value() -> u32 {
    unsafe {
        SYSTICK.rvr().reload()
    }
}

pub fn set_reload_value(value: u32) {
    unsafe {
        SYSTICK.set_rvr(Rvr(0).set_reload(value))
    }
}

pub fn current_value() -> u32 {
    unsafe {
        SYSTICK.cvr().current()
    }
}

pub fn set_current_value(value: u32) {
    unsafe {
        SYSTICK.set_cvr(Cvr(0).set_current(value))
    }
}

pub fn no_reference_clock() -> bool {
    unsafe {
        SYSTICK.calib().noref() != 0
    }
}

pub fn skew() -> bool {
    unsafe {
        SYSTICK.calib().skew() != 0
    }
}

pub fn ten_ms() -> u32 {
    unsafe {
        SYSTICK.calib().tenms()
    }
}