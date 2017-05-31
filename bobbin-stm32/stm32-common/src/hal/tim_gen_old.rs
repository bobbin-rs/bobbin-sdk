pub use ::chip::tim_gen::*;
// use ::hal::rcc;

pub enum Direction {
    Up = 0,
    Down = 1,
}

pub enum OcMode {
    Frozen =        0b0000,
    Active =        0b0001,
    Inactive =      0b0010,
    Toggle =        0b0011,
    ForceInactive = 0b0100,
    ForceActive =   0b0101,
    Pwm1 =          0b0110,
    Pwm2 =          0b0111,
    CombinedPwm1 =  0b1100,
    CombinedPwm2 =  0b1101,
    AsymPwm1 =      0b1110,
    AsymPwm2 =      0b1111,
}

pub enum CcSelect {
    Output   = 0b00,
    InputA   = 0b01,
    InputB   = 0b10,
    InputTRC = 0b11,
}

pub const CH1: usize = 0;
pub const CH2: usize = 1;
pub const CH3: usize = 2;
pub const CH4: usize = 3;


pub struct TimGenDevice {
    tim: TimGenImpl,
}

pub fn device(tim: TimGenImpl) -> TimGenDevice {    
    TimGenDevice { tim: tim }
}

impl TimGenDevice {
    pub fn init(&self) {
        // rcc::set_tim_gen_enabled(self.tim, true);
    }

    pub fn tim(&self) -> TimGenImpl {
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
        let tim = self.tim;
        unsafe {
            tim.with_cr1(|r| r.set_cen(value))
        }
    }

    pub fn direction(&self) -> Direction {
        let tim = self.tim;
        unsafe {
            match tim.cr1().dir() {
                0 => Direction::Down,
                1 => Direction::Up,
                _ => panic!("Unexpected Direction"),
            }
        }
    }

    pub fn set_direction(&self, value: Direction) {
        let tim = self.tim;
        unsafe {
            tim.with_cr1(|r| r.set_dir(value as u32))
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
        let tim = self.tim;
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
        let tim = self.tim;
        unsafe {
            tim.with_dier(|r| r.set_uie(value))
        }
    }

    pub fn cc_interrupt_enabled(&self, index: usize) -> bool {
        let tim = self.tim;
        unsafe { 
            tim.dier().ccie(index) != 0
        }
    }

    pub fn set_cc_interrupt_enabled(&self, index: usize, value: bool) {
        let value = if value { 1 } else { 0 };
        let tim = self.tim;
        unsafe {
            tim.with_dier(|r| r.set_ccie(index, value))
        }
    }            

    pub fn trigger_interrupt_enabled(&self) -> bool {
        let tim = self.tim;
        unsafe { 
            tim.dier().tie() != 0
        }
    }

    pub fn set_trigger_interrupt_enabled(&self, value: bool) {
        let value = if value { 1 } else { 0 };
        let tim = self.tim;
        unsafe {
            tim.with_dier(|r| r.set_tie(value))
        }
    }    

    pub fn update_interrupt_flag(&self) -> bool {
        let tim = self.tim;
        unsafe { 
            tim.sr().uif() != 0
        }
    }

    pub fn clr_update_interrupt_flag(&self) {
        let tim = self.tim;
        unsafe {
            tim.with_sr(|r| r.set_uif(0))
        }        
    }

    pub fn cc_interrupt_flag(&self, index: usize) -> bool {
        let tim = self.tim;
        unsafe { 
            tim.sr().ccif(index) != 0
        }
    }

    pub fn clr_cc_interrupt_flag(&self, index: usize) {
        let tim = self.tim;
        unsafe {
            tim.with_sr(|r| r.set_ccif(index, 0))
        }        
    }

    pub fn set_trigger_event(&self) {
        let tim = self.tim;
        unsafe {
            tim.set_egr(Egr(0).set_tg(1))
        }
    }

    // Event Generation

    pub fn set_ccg_event(&self, index: usize) {
        let tim = self.tim;
        unsafe {
            tim.set_egr(Egr(0).set_ccg(index, 1))
        }
    }    

    pub fn set_update_event(&self) {
        let tim = self.tim;
        unsafe {
            tim.set_egr(Egr(0).set_ug(1))
        }
    }    

    // Capture / Compare Mode

    pub fn set_ccs(&self, index: usize, value: CcSelect) {
        let tim = self.tim;
        unsafe {
            match index {
                0...1 => tim.with_ccmr_output(0, |r| r.set_ccs(index, value as u32)),
                2...3 => tim.with_ccmr_output(1, |r| r.set_ccs(index - 2, value as u32)),
                _ => panic!("Invalid channel index"),
            }
        }
    }

    pub fn set_ocm(&self, index: usize, value: OcMode) {
        let tim = self.tim;
        unsafe {
            let value = value as u32;
            let v012 = value & 0b111;
            let v3 = value >> 3;
            match index {
                0...1 => tim.with_ccmr_output(0, |r| r.set_ocm(index, v012).set_ocm_3(index, v3)),
                2...3 => tim.with_ccmr_output(1, |r| r.set_ocm(index - 2, v012).set_ocm_3(index - 2, v3)),
                _ => panic!("Invalid channel index"),
            }
        }
    }

    pub fn set_ocfe(&self, index: usize, value: bool) {
        let value = if value { 1 } else { 0 };
        let tim = self.tim;
        unsafe {
            match index {
                0...1 => tim.with_ccmr_output(0, |r| r.set_ocfe(index, value as u32)),
                2...3 => tim.with_ccmr_output(1, |r| r.set_ocfe(index - 2, value as u32)),
                _ => panic!("Invalid channel index"),
            }
        }
    }

    pub fn set_ocpe(&self, index: usize, value: bool) {
        let value = if value { 1 } else { 0 };
        let tim = self.tim;
        unsafe {
            match index {
                0...1 => tim.with_ccmr_output(0, |r| r.set_ocpe(index, value as u32)),
                2...3 => tim.with_ccmr_output(1, |r| r.set_ocpe(index - 2, value as u32)),
                _ => panic!("Invalid channel index"),
            }
        }
    }

    pub fn set_occe(&self, index: usize, value: bool) {
        let value = if value { 1 } else { 0 };
        let tim = self.tim;
        unsafe {
            match index {
                0...1 => tim.with_ccmr_output(0, |r| r.set_occe(index, value as u32)),
                2...3 => tim.with_ccmr_output(1, |r| r.set_occe(index - 2, value as u32)),
                _ => panic!("Invalid channel index"),
            }
        }
    }    

    // CC Enable

    pub fn set_cce(&self, index: usize, value: bool) {
        let value = if value { 1 } else { 0 };
        let tim = self.tim;
        unsafe {
            tim.with_ccer(|r| r.set_cce(index, value))
        }
    }


    pub fn set_ccp(&self, index: usize, value: bool) {
        let value = if value { 1 } else { 0 };
        let tim = self.tim;
        unsafe {
            tim.with_ccer(|r| r.set_ccp(index, value))
        }
    }

    pub fn set_ccnp(&self, index: usize, value: bool) {
        let value = if value { 1 } else { 0 };
        let tim = self.tim;
        unsafe {
            tim.with_ccer(|r| r.set_ccnp(index, value))
        }
    }


    pub fn prescaler(&self) -> u16 {
        let tim = self.tim;
        unsafe {
            tim.psc().psc() as u16
        }
    }

    pub fn set_prescaler(&self, value: u16) {
        let tim = self.tim;
        unsafe {
            tim.set_psc(Psc(0).set_psc(value as u32))
        }
    }

    // TODO: 16 bit devices should use 16 bit values

    pub fn counter(&self) -> u32 {
        let tim = self.tim;
        unsafe {
            tim.cnt().0 as u32
        }
    }

    pub fn set_counter(&self, value: u32) {
        let tim = self.tim;
        unsafe {
            tim.set_cnt(Cnt(value))
        }
    }

    pub fn auto_reload(&self) -> u32 {
        let tim = self.tim;
        unsafe {
            tim.arr().0 as u32
        }
    }

    pub fn set_auto_reload(&self, value: u32) {
        let tim = self.tim;
        unsafe {
            tim.set_arr(Arr(value))
        }
    }


    pub fn capture_compare(&self, index: usize) -> u32 {
        let tim = self.tim;
        unsafe {
            tim.ccr(index).0
        }
    }

    pub fn set_capture_compare(&self, index: usize, value: u32) {
        let tim = self.tim;
        unsafe {
            tim.set_ccr(index, Ccr(value))
        }
    }

}