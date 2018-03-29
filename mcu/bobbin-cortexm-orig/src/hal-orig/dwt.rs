pub use ::chip::dwt::*;

impl Dwt {
    // Cycle Count

    #[inline]
    pub fn cycle_count_enabled(&self) -> bool {
        (self.ctrl().0 | 1) != 0
    }

    #[inline]
    pub fn set_cycle_count_enabled(&self, value: bool) -> &Self {
        let mut ctrl = self.ctrl().0;
        if value {
            ctrl |= 1
        } else {
            ctrl &= !1;            
        }        
        self.set_ctrl(|_| Ctrl(ctrl))
    }

    #[inline]
    pub fn enable_cycle_count(&self) -> &Self {
        self.set_cycle_count_enabled(true)
    }

    #[inline]
    pub fn disable_cycle_count(&self) -> &Self {
        self.set_cycle_count_enabled(false)
    }

    #[inline]
    pub fn cycle_count(&self) -> u32 {
        self.cyccnt().0
    }

    #[inline]
    pub fn clr_cycle_count(&self) -> &Self {
        self.set_cyccnt(|_| Cyccnt(0))
    }

    #[inline]
    pub fn set_cycle_count(&self, value: u32) -> &Self {
        self.set_cyccnt(|_| Cyccnt(value))
    }

    #[inline]
    pub fn with_cycle_count<F: FnOnce()>(&self, f: F) -> u32 {
        self
            .set_cycle_count_enabled(false)
            .set_cycle_count(0)
            .set_cycle_count_enabled(true);
        f();
        self
            .set_cycle_count_enabled(false)
            .cycle_count()        
    }

    // Sleep Count

    #[inline]
    pub fn sleep_count_enabled(&self) -> bool {
        (self.ctrl().0 | 1 << 19) != 0
    }

    #[inline]
    pub fn set_sleep_count_enabled(&self, value: bool) -> &Self {
        let mut ctrl = self.ctrl().0;
        if value {
            ctrl |= 1 << 19
        } else {
            ctrl &= !(1 << 19);            
        }        
        self.set_ctrl(|_| Ctrl(ctrl))
    }

    #[inline]
    pub fn enable_sleep_count(&self) -> &Self {
        self.set_sleep_count_enabled(true)
    }

    #[inline]
    pub fn disable_sleep_count(&self) -> &Self {
        self.set_sleep_count_enabled(false)
    }

    #[inline]
    pub fn sleep_count(&self) -> u32 {
        self.sleepcnt().0
    }

    #[inline]
    pub fn clr_sleep_count(&self) -> &Self {
        self.set_sleepcnt(|_| Sleepcnt(0))
    }

    #[inline]
    pub fn set_sleep_count(&self, value: u32) -> &Self {
        self.set_sleepcnt(|_| Sleepcnt(value))
    }

    #[inline]
    pub fn with_sleep_count<F: FnOnce()>(&self, f: F) -> u32 {
        self
            .set_sleep_count_enabled(false)
            .set_sleep_count(0)
            .set_sleep_count_enabled(true);
        f();
        self
            .set_sleep_count_enabled(false)
            .sleep_count()        
    }    

}