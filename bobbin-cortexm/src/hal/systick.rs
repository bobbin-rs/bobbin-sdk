//! Extends the ```chip::systick``` module.
//! See [4.4. System timer, SysTick](http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0552a/Babieigh.html)

use bobbin_common::bits::*;
pub use ::chip::systick::*;
pub use ::chip::exc::{Handler, EXC_SYSTICK};

/// The clock source to be used by Systick.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ClockSource {
    /// Use the external clock source.
    External = 0,
    /// Use the internal clock source.
    Internal = 1,
}

/// Sets a handler for the Systick exception.
pub fn set_handler(handler: Option<Handler>) {
    EXC_SYSTICK.set_handler(handler);
}

/// Returns true if timer counted to 0 since the last time this was read.
pub fn count_flag() -> bool {
    SYSTICK.csr().countflag() != 0
}

/// Returns the clock source in use.
///
/// ```
/// # extern crate bobbin_common;
/// # extern crate bobbin_cortexm;
/// # use bobbin_common::rw::*;
/// # use bobbin_cortexm::hal::systick::*;
///
/// fn main() {
///    # add_region(0xe000_e000, 0x1000);
///    // ClockSource::External is the default.
///    assert_eq!(clock_source(), ClockSource::External);
/// }
pub fn clock_source() -> ClockSource {
    match SYSTICK.csr().clksource() {
        U1::B0 => ClockSource::External,
        U1::B1 => ClockSource::Internal,
    }
}

/// Set the clock source to use.
///
/// ```
/// # extern crate bobbin_common;
/// # extern crate bobbin_cortexm;
/// # use bobbin_common::rw::*;
/// # use bobbin_cortexm::hal::systick::*;
///
/// fn main() {
///    # add_region(0xe000_e000, 0x1000);
///    assert_eq!(clock_source(), ClockSource::External);
///    set_clock_source(ClockSource::Internal);
///    assert_eq!(clock_source(), ClockSource::Internal);
/// }
pub fn set_clock_source(value: ClockSource) {
    SYSTICK.with_csr(|r| r.set_clksource(value as u32));
}


/// Returns true if the SysTick exception is enabled.
pub fn tick_interrupt() -> bool {
    SYSTICK.csr().tickint() != 0
}

/// True enables the SysTick exception, false disables.
pub fn set_tick_interrupt(value: bool) {
    let value = if value { 1 } else { 0 };
    SYSTICK.with_csr(|r| r.set_tickint(value));
}

/// Returns true if the Systick counter is enabled.
pub fn enabled() -> bool {
    SYSTICK.csr().enable() != 0
}

/// True enables the SysTick counter, false disables.
pub fn set_enabled(value: bool) {
    let value = if value { 1 } else { 0 };
    SYSTICK.with_csr(|r| r.set_enable(value));
}

/// Returns the value that will be loaded into the counter
/// when the counter is enabled and reaches 0.
pub fn reload_value() -> u32 {
    SYSTICK.rvr().reload().into()
}

/// Sets the value that will be loaded into the counter
/// when the counter is enabled and reaches 0.
pub fn set_reload_value(value: u32) {
    SYSTICK.set_rvr(|r| r.set_reload(value));
}

/// Returns the current value of the counter.
pub fn current_value() -> u32 {
    SYSTICK.cvr().current().into()
}

/// Sets the current value of the counter.
pub fn set_current_value(value: u32) {
    SYSTICK.set_cvr(|r| r.set_current(value));
}

/// Returns true if no reference clock is provided.
pub fn no_reference_clock() -> bool {
    SYSTICK.calib().noref() != 0
}

/// Returns true if ten_ms() is exact.
pub fn skew() -> bool {
    SYSTICK.calib().skew() != 0
}

/// Returns the reload value for 10ms (100Hz) timing. If the value reads
/// zero, the calibration value is not known.
pub fn ten_ms() -> u32 {
    SYSTICK.calib().tenms().into()
}