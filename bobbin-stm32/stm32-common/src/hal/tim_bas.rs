pub use ::chip::tim_bas::*;
// use ::hal::rcc;

pub struct TimBasDevice {
    tim: TimBas,
}

pub fn device(tim: TimBas) -> TimBasDevice {    
    TimBasDevice { tim: tim }
}

impl TimBasDevice {
    pub fn init(&self) {
        // rcc::set_tim_bas_enabled(self.tim, true);
    }

    pub fn tim(&self) -> TimBas {
        self.tim
    }

    pub fn enabled(&self) -> bool {
        let tim = self.tim;
        unsafe { 
            tim.cr1().cen() != 0
        }
    }

    pub fn set_enabled(&self, value: bool) {
        let value = if value { 1 } else { 0 };
        let mut tim = self.tim;
        unsafe {
            tim.with_cr1(|r| r.set_cen(value))
        }
    }

    pub fn one_pulse_mode(&self) -> bool {
        let tim = self.tim;
        unsafe {
            tim.cr1().opm() != 0
        }
    }

    pub fn set_one_pulse_mode(&self, value: bool) {
        let value = if value { 1 } else { 0 };
        let mut tim = self.tim;
        unsafe {
            tim.with_cr1(|r| r.set_opm(value))
        }
    }

    pub fn update_interrupt_enabled(&self) -> bool {
        let tim = self.tim;
        unsafe { 
            tim.dier().uie() != 0
        }
    }

    pub fn set_update_interrupt_enabled(&self, value: bool) {
        let value = if value { 1 } else { 0 };
        let mut tim = self.tim;
        unsafe {
            tim.with_dier(|r| r.set_uie(value))
        }
    }

    pub fn update_interrupt_flag(&self) -> bool {
        let tim = self.tim;
        unsafe { 
            tim.sr().uif() != 0
        }
    }

    pub fn clr_update_interrupt_flag(&self) {
        let mut tim = self.tim;
        unsafe {
            tim.with_sr(|r| r.set_uif(0))
        }        
    }

    pub fn counter(&self) -> u16 {
        let tim = self.tim;
        unsafe {
            tim.cnt().cnt() as u16
        }
    }

    pub fn set_counter(&self, value: u16) {
        let mut tim = self.tim;
        unsafe {
            tim.set_cnt(Cnt(0).set_cnt(value as u32))
        }
    }

    pub fn prescaler(&self) -> u16 {
        let tim = self.tim;
        unsafe {
            tim.psc().psc() as u16
        }
    }

    pub fn set_prescaler(&self, value: u16) {
        let mut tim = self.tim;
        unsafe {
            tim.set_psc(Psc(0).set_psc(value as u32))
        }
    }


    pub fn auto_reload(&self) -> u16 {
        let tim = self.tim;
        unsafe {
            tim.arr().arr() as u16
        }
    }

    pub fn set_auto_reload(&self, value: u16) {
        let mut tim = self.tim;
        unsafe {
            tim.set_arr(Arr(0).set_arr(value as u32))
        }
    }
}