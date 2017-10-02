//! Extends the ```chip::systick``` module.
//! See [4.4. System timer, SysTick](http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0552a/Babieigh.html)


use bobbin_common::bits::*;
pub use bobbin_common::HandleIrq;
pub use bobbin_common::timer::*;
pub use ::chip::systick::*;
pub use ::chip::exc::{Handler, EXC_SYSTICK};

pub trait HandleSystick {
    fn handle_systick(&self);
}


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
    pub fn tick_interrupt(&self) -> bool {
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
    pub fn set_enabled(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.with_csr(|r| r.set_enable(value))
    }

    /// Returns the value that will be loaded into the counter
    /// when the counter is enabled and reaches 0.
    pub fn reload_value(&self) -> U24 {
        self.rvr().reload()
    }

    /// Sets the value that will be loaded into the counter
    /// when the counter is enabled and reaches 0.
    pub fn set_reload_value<V: Into<U24>>(&self, value: V) -> &Self {
        self.set_rvr(|r| r.set_reload(value))
    }

    /// Returns the current value of the counter.
    pub fn current_value(&self) -> U24 {
        self.cvr().current()
    }

    /// Sets the current value of the counter.
    pub fn set_current_value<V: Into<U24>>(&self, value: V) -> &Self {
        self.set_cvr(|r| r.set_current(value))
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
    pub fn ten_ms(&self) -> U24 {
        self.calib().tenms()
    }    

    /// Returns true if timer counted to 0 since the last time this was read.
    pub fn count_flag(&self) -> bool {
        self.csr().test_countflag()
    }
    
    pub fn clr_count_flag(&self) -> &Self {
        let _ = self.csr().countflag();
        self
    }
    // /// Sets a handler for the Systick exception.
    // pub fn set_handler(&self, handler: Option<Handler>) -> &Self {
    //     EXC_SYSTICK.set_handler(handler);
    //     self
    // }

    pub fn set_handler<'a, F: HandleSystick>(&self, f: &F) {
        static mut HANDLER: Option<usize> = None;                
        unsafe { 
            assert!(HANDLER.is_none(), "Irq is already wrapping a function");
            HANDLER = Some(f as *const F as usize)
        }
        extern "C" fn wrapper<W: HandleSystick>() {
            unsafe { (*(HANDLER.unwrap() as *const W)).handle_systick() }
        }
        EXC_SYSTICK.set_handler(Some(wrapper::<F>));                
    }    
}

impl<V: Into<U24>> Start<V> for Systick {
    fn start(&self, value: V) -> &Self {
        self.start_down(value)
    }
}

impl<V: Into<U24>> StartDown<V> for Systick {
    fn start_down(&self, value: V) -> &Self {                
        self
            .set_enabled(false)
            .clr_count_flag()
            .set_reload_value(value)
            .set_current_value(0)
            .set_enabled(true)
    }
}

impl Running for Systick {
    fn running(&self) -> bool {
        self.enabled()
    }
}

impl Stop for Systick {
    fn stop(&self) -> &Self {
        self.set_enabled(false)
    }
}

impl Timeout for Systick {
    fn test_timeout(&self) -> bool {
        self.count_flag()
    }

    fn clr_timeout(&self) -> &Self {
        self.clr_count_flag()
    }
}

impl Counter<U24> for Systick {
    fn counter(&self) -> U24 {
        self.current_value()
    }
}

impl ClearCounter for Systick {
    fn clr_counter(&self) -> &Self {
        self.set_current_value(0)
    }
}

impl Period<U24> for Systick {
    fn period(&self) -> U24 {
        self.reload_value()
    }
}

impl<V: Into<U24>> SetPeriod<V> for Systick {
    fn set_period(&self, value: V) -> &Self {
        self.set_reload_value(value)
    }
}

impl<V: Into<U24>> Delay<V> for Systick {
    fn delay(&self, value: V) -> &Self {
        self.start_down(value).wait_timeout()
    }
}

impl Elapsed<U24> for Systick {
    fn elapsed<F: FnOnce()>(&self, f: F) -> Option<U24> {
        const MAX: u32 = (1u32 << 24) - 1u32;
        self.clr_count_flag().set_period(MAX).clr_counter().set_enabled(true);
        f();
        let value = self.set_enabled(false).counter().value();
        if self.count_flag() {
            None
        } else {
            unsafe {            
                Some(U24::from_u32_unchecked(MAX-value))
            }
        }
    }
}


pub fn test_systick(s: &Systick, source: ClockSource) {
    let reload_value = 10000u32;

    s.set_enabled(false);
    assert!(!s.enabled());

    s.set_clock_source(source);
    assert_eq!(s.clock_source(), source);

    s.set_reload_value(reload_value);
    assert_eq!(s.reload_value().value(), reload_value);

    s.set_current_value(0);
    assert_eq!(s.current_value(), 0);

    let _ = s.count_flag();
    assert!(!s.count_flag());


    let mut value_min = s.current_value().value();

    s.set_enabled(true);
    assert!(s.enabled());    
    assert!(s.current_value().value() > 0);
    while !s.count_flag() {
        let v = s.current_value().value();
        if v < value_min {
            value_min = v;
        }
    }
    assert!(value_min < reload_value);
    s.set_enabled(false);
    
}
