//! Extends the ```chip::systick``` module.
//! See [4.4. System timer, SysTick](http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0552a/Babieigh.html)

use bobbin_common::bits::*;
pub use ::chip::systick::*;
pub use ::chip::exc::{Handler, EXC_SYSTICK};


/// The clock source to be used by self.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ClockSource {
    /// Use the external clock source.
    External = 0,
    /// Use the internal clock source.
    Internal = 1,
}

impl Systick {
    pub fn clock_source(&self) -> ClockSource {
        match self.csr().clksource() {
            U1::B0 => ClockSource::External,
            U1::B1 => ClockSource::Internal,
        }
    }
        
    // Set the SysTick clock source.
    pub fn set_clock_source(&self, value: ClockSource) {
        self.with_csr(|r| r.set_clksource(value as u32));
    }


    /// Returns true if the SysTick exception is enabled.
    pub fn tick_interrupt(&self, ) -> bool {
        self.csr().tickint() != 0
    }

    /// True enables the SysTick exception, false disables.
    pub fn set_tick_interrupt(&self, value: bool) {
        let value = if value { 1 } else { 0 };
        self.with_csr(|r| r.set_tickint(value));
    }

    /// Returns true if the Systick counter is enabled.
    pub fn enabled(&self) -> bool {
        self.csr().enable() != 0
    }

    /// True enables the SysTick counter, false disables.
    pub fn set_enabled(&self, value: bool) {
        let value = if value { 1 } else { 0 };
        self.with_csr(|r| r.set_enable(value));
    }

    /// Returns the value that will be loaded into the counter
    /// when the counter is enabled and reaches 0.
    pub fn reload_value(&self) -> u32 {
        self.rvr().reload().into()
    }

    /// Sets the value that will be loaded into the counter
    /// when the counter is enabled and reaches 0.
    pub fn set_reload_value(&self, value: u32) {
        self.set_rvr(|r| r.set_reload(value));
    }

    /// Returns the current value of the counter.
    pub fn current_value(&self) -> u32 {
        self.cvr().current().into()
    }

    /// Sets the current value of the counter.
    pub fn set_current_value(&self, value: u32) {
        self.set_cvr(|r| r.set_current(value));
    }

    /// Returns true if no reference clock is provided.
    pub fn no_reference_clock(&self) -> bool {
        self.calib().noref() != 0
    }

    /// Returns true if ten_ms() is exact.
    pub fn skew(&self) -> bool {
        self.calib().skew() != 0
    }

    /// Returns the reload value for 10ms (100Hz) timing. If the value reads
    /// zero, the calibration value is not known.
    pub fn ten_ms(&self) -> u32 {
        self.calib().tenms().into()
    }    

    /// Returns true if timer counted to 0 since the last time this was read.
    pub fn count_flag(&self) -> bool {
        self.csr().test_countflag()
    }
    
    /// Sets a handler for the Systick exception.
    pub fn set_handler(&self, handler: Option<Handler>) {
        EXC_SYSTICK.set_handler(handler);
    }
}

pub fn test_systick(s: &Systick, source: ClockSource) {
    let reload_value = 10000;

    s.set_enabled(false);
    assert!(!s.enabled());

    s.set_clock_source(source);
    assert_eq!(s.clock_source(), source);

    s.set_reload_value(reload_value);
    assert_eq!(s.reload_value(), reload_value);

    s.set_current_value(0);
    assert_eq!(s.current_value(), 0);

    let _ = s.count_flag();
    assert!(!s.count_flag());


    let mut value_min = s.current_value();

    s.set_enabled(true);
    assert!(s.enabled());    
    assert!(s.current_value() > 0);
    while !s.count_flag() {
        let v = s.current_value();
        if v < value_min {
            value_min = v;
        }
    }
    assert!(value_min < reload_value);
    s.set_enabled(false);
    
}
