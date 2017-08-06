pub use chip::tim_bas::*;

pub trait TimBasExt {    
    fn enabled(&self) -> bool;
    fn set_enabled(&self, bool) -> &Self;
    fn one_pulse_mode(&self) -> bool;
    fn set_one_pulse_mode(&self, value: bool) -> &Self;
    fn update_interrupt_flag(&self) -> bool;
    fn clr_update_interrupt_flag(&self) -> &Self;
    fn counter(&self) -> u16;
    fn set_counter(&self, value: u16) -> &Self;
    fn prescaler(&self) -> u16;
    fn set_prescaler(&self, value: u16) -> &Self;
    fn reload(&self) -> u16;
    fn set_reload(&self, value: u16) -> &Self;
}

impl<T> TimBasExt for Periph<T> {
    fn enabled(&self) -> bool {
        self.cr1().cen() != 0
    }
    fn set_enabled(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.with_cr1(|r| r.set_cen(value));
        self
    }

    fn one_pulse_mode(&self) -> bool {
        self.cr1().opm() != 0
    }
    fn set_one_pulse_mode(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.with_cr1(|r| r.set_opm(value));
        self
    }
    fn update_interrupt_flag(&self) -> bool {
        self.sr().uif() != 0
    }
    fn clr_update_interrupt_flag(&self) -> &Self {
        self.set_sr(Sr(0).set_uif(0))
    }

    fn counter(&self) -> u16 {
        self.cnt().cnt().into()
    }
    fn set_counter(&self, value: u16) -> &Self {
        self.set_cnt(Cnt(0).set_cnt(value))        
    }

    fn prescaler(&self) -> u16 {
        self.psc().psc().into()
    }
    fn set_prescaler(&self, value: u16) -> &Self {
        self.set_psc(Psc(0).set_psc(value))        
    }

    fn reload(&self) -> u16 {
        self.arr().arr().into()
    }
    fn set_reload(&self, value: u16) -> &Self {
        self.set_arr(Arr(0).set_arr(value))        
    }
}