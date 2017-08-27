pub use chip::tim_bas::*;

impl TimBasPeriph {
    pub fn enabled(&self) -> bool {
        self.cr1().cen() != 0
    }
    pub fn set_enabled(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.with_cr1(|r| r.set_cen(value));
        self
    }

    pub fn one_pulse_mode(&self) -> bool {
        self.cr1().opm() != 0
    }
    pub fn set_one_pulse_mode(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.with_cr1(|r| r.set_opm(value));
        self
    }
    pub fn update_interrupt_flag(&self) -> bool {
        self.sr().uif() != 0
    }
    pub fn clr_update_interrupt_flag(&self) -> &Self {
        self.set_sr(|r| r.set_uif(0))
    }

    pub fn counter(&self) -> u16 {
        self.cnt().cnt().into()
    }
    pub fn set_counter(&self, value: u16) -> &Self {
        self.set_cnt(|r| r.set_cnt(value))        
    }

    pub fn prescaler(&self) -> u16 {
        self.psc().psc().into()
    }
    pub fn set_prescaler(&self, value: u16) -> &Self {
        self.set_psc(|r| r.set_psc(value))        
    }

    pub fn reload(&self) -> u16 {
        self.arr().arr().into()
    }
    pub fn set_reload(&self, value: u16) -> &Self {
        self.set_arr(|r| r.set_arr(value))        
    }
}