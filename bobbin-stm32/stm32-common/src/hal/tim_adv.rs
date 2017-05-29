pub use ::chip::tim_adv::*;
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
pub const CH5: usize = 4;
pub const CH6: usize = 5;


pub struct TimAdvDevice {
    tim: TimAdv,
}

pub fn device(tim: TimAdv) -> TimAdvDevice {    
    TimAdvDevice { tim: tim }
}

impl TimAdvDevice {
    pub fn init(&self) {
        // rcc::set_tim_adv_enabled(self.tim, true);
    }

    pub fn tim(&self) -> TimAdv {
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
        let mut tim = self.tim;
        unsafe {
            tim.with_cr1(|r| r.set_dir(value as u32))
        }    }

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

    // Interrupt Enable Register

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

    pub fn cc_interrupt_enabled(&self, index: usize) -> bool {
        let tim = self.tim;
        unsafe { 
            match index {
                0 => tim.dier().cc1ie() != 0,
                1 => tim.dier().cc2ie() != 0,
                2 => tim.dier().cc3ie() != 0,
                3 => tim.dier().cc4ie() != 0,
                _ => panic!("Only Ch 1..4 supported"),
            }
            
        }
    }

    pub fn set_cc_interrupt_enabled(&self, index: usize, value: bool) {
        let value = if value { 1 } else { 0 };
        let mut tim = self.tim;
        unsafe {
            match index {
                0 => tim.with_dier(|r| r.set_cc1ie(value)),
                1 => tim.with_dier(|r| r.set_cc2ie(value)),
                2 => tim.with_dier(|r| r.set_cc3ie(value)),
                3 => tim.with_dier(|r| r.set_cc4ie(value)),
                _ => panic!("Only Ch 1..4 supported"),
            }
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
        let mut tim = self.tim;
        unsafe {
            tim.with_dier(|r| r.set_tie(value))
        }
    }    

    // Status Register

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

    pub fn cc_interrupt_flag(&self, index: usize) -> bool {
        let tim = self.tim;
        unsafe { 
            match index {
                0 => tim.sr().cc1if() != 0,
                1 => tim.sr().cc2if() != 0,
                2 => tim.sr().cc3if() != 0,
                3 => tim.sr().cc4if() != 0,
                4 => tim.sr().cc5if() != 0,
                5 => tim.sr().cc6if() != 0,
                _ => panic!("Only Ch 1..6 supported"),
            }
        }
    }

    pub fn clr_cc_interrupt_flag(&self, index: usize) {
        let mut tim = self.tim;
        unsafe {
            match index {
                0 => tim.with_sr(|r| r.set_cc1if(0)),
                1 => tim.with_sr(|r| r.set_cc2if(0)),
                2 => tim.with_sr(|r| r.set_cc3if(0)),
                3 => tim.with_sr(|r| r.set_cc4if(0)),
                4 => tim.with_sr(|r| r.set_cc5if(0)),
                5 => tim.with_sr(|r| r.set_cc6if(0)),
                _ => panic!("Only Ch 1..6 supported"),
            }                
        }        
    }

    pub fn set_trigger_event(&self) {
        let mut tim = self.tim;
        unsafe {
            tim.set_egr(Egr(0).set_tg(1))
        }
    }

    // Event Generation

    pub fn set_ccg_event(&self, index: usize) {
        let mut tim = self.tim;
        unsafe {
            match index {
                0 => tim.set_egr(Egr(0).set_cc1g(1)),
                1 => tim.set_egr(Egr(0).set_cc2g(1)),
                2 => tim.set_egr(Egr(0).set_cc3g(1)),
                3 => tim.set_egr(Egr(0).set_cc4g(1)),
                _ => panic!("Only Ch 1..4 supported"),
            }
        }
    }    

    pub fn set_update_event(&self) {
        let mut tim = self.tim;
        unsafe {
            tim.set_egr(Egr(0).set_ug(1))
        }
    }    

    // Capture / Compare Mode

    pub fn set_ccs(&self, index: usize, value: CcSelect) {
        let mut tim = self.tim;
        unsafe {
            match index {
                0 => tim.with_ccmr1_output(|r| r.set_cc1s(value as u32)),
                1 => tim.with_ccmr1_output(|r| r.set_cc2s(value as u32)),
                2 => tim.with_ccmr2_output(|r| r.set_cc3s(value as u32)),
                3 => tim.with_ccmr2_output(|r| r.set_cc4s(value as u32)),
                _ => panic!("Only Ch 1..4 supported"),
            }
        }
    }

    pub fn set_ocm(&self, index: usize, value: OcMode) {
        let mut tim = self.tim;
        unsafe {
            let value = value as u32;
            let v012 = value & 0b111;
            let v3 = value >> 3;
            match index {
                0 => tim.with_ccmr1_output(|r| r.set_oc1m(v012).set_oc1m_3(v3)),
                1 => tim.with_ccmr1_output(|r| r.set_oc2m(v012).set_oc2m_3(v3)),
                2 => tim.with_ccmr2_output(|r| r.set_oc3m(v012).set_oc3m_3(v3)),
                3 => tim.with_ccmr2_output(|r| r.set_oc4m(v012).set_oc4m_3(v3)),
                _ => panic!("Only Ch 1..6 supported"),
            }
        }
    }

    pub fn set_ocfe(&self, index: usize, value: bool) {
        let value = if value { 1 } else { 0 };
        let mut tim = self.tim;
        unsafe {
            match index {
                0 => tim.with_ccmr1_output(|r| r.set_oc1fe(value as u32)),
                1 => tim.with_ccmr1_output(|r| r.set_oc2fe(value as u32)),
                2 => tim.with_ccmr2_output(|r| r.set_oc3fe(value as u32)),
                3 => tim.with_ccmr2_output(|r| r.set_oc4fe(value as u32)),
                4 => tim.with_ccmr3_output(|r| r.set_oc5fe(value as u32)),
                5 => tim.with_ccmr3_output(|r| r.set_oc6fe(value as u32)),
                _ => panic!("Only Ch 1..6 supported"),
            }
        }
    }

    pub fn set_ocpe(&self, index: usize, value: bool) {
        let value = if value { 1 } else { 0 };
        let mut tim = self.tim;
        unsafe {
            match index {
                0 => tim.with_ccmr1_output(|r| r.set_oc1pe(value as u32)),
                1 => tim.with_ccmr1_output(|r| r.set_oc2pe(value as u32)),
                2 => tim.with_ccmr2_output(|r| r.set_oc3pe(value as u32)),
                3 => tim.with_ccmr2_output(|r| r.set_oc4pe(value as u32)),
                4 => tim.with_ccmr3_output(|r| r.set_oc5pe(value as u32)),
                5 => tim.with_ccmr3_output(|r| r.set_oc6pe(value as u32)),
                _ => panic!("Only Ch 1..6 supported"),
            }
        }
    }

    pub fn set_occe(&self, index: usize, value: bool) {
        let value = if value { 1 } else { 0 };
        let mut tim = self.tim;
        unsafe {
            match index {
                0 => tim.with_ccmr1_output(|r| r.set_oc1ce(value as u32)),
                1 => tim.with_ccmr1_output(|r| r.set_oc2ce(value as u32)),
                2 => tim.with_ccmr2_output(|r| r.set_oc3ce(value as u32)),
                3 => tim.with_ccmr2_output(|r| r.set_oc4ce(value as u32)),
                4 => tim.with_ccmr3_output(|r| r.set_oc5ce(value as u32)),
                5 => tim.with_ccmr3_output(|r| r.set_oc6ce(value as u32)),                
                _ => panic!("Only Ch 1..6 supported"),
            }
        }
    }    

    // CC Enable

    pub fn set_cce(&self, index: usize, value: bool) {
        let value = if value { 1 } else { 0 };
        let mut tim = self.tim;
        unsafe {
            match index {
                0 => tim.with_ccer(|r| r.set_cc1e(value)),
                1 => tim.with_ccer(|r| r.set_cc2e(value)),
                2 => tim.with_ccer(|r| r.set_cc3e(value)),
                3 => tim.with_ccer(|r| r.set_cc4e(value)),
                4 => tim.with_ccer(|r| r.set_cc5e(value)),
                5 => tim.with_ccer(|r| r.set_cc6e(value)),
                _ => panic!("Only Ch 1..6 supported"),
            }
        }
    }


    pub fn set_ccp(&self, index: usize, value: bool) {
        let value = if value { 1 } else { 0 };
        let mut tim = self.tim;
        unsafe {
            match index {
                0 => tim.with_ccer(|r| r.set_cc1p(value)),
                1 => tim.with_ccer(|r| r.set_cc2p(value)),
                2 => tim.with_ccer(|r| r.set_cc3p(value)),
                3 => tim.with_ccer(|r| r.set_cc4p(value)),
                4 => tim.with_ccer(|r| r.set_cc5p(value)),
                5 => tim.with_ccer(|r| r.set_cc6p(value)),
                _ => panic!("Only Ch 1..6 supported"),
            }
        }
    }

    pub fn set_ccnp(&self, index: usize, value: bool) {
        let value = if value { 1 } else { 0 };
        let mut tim = self.tim;
        unsafe {
            match index {
                0 => tim.with_ccer(|r| r.set_cc1np(value)),
                1 => tim.with_ccer(|r| r.set_cc2np(value)),
                2 => tim.with_ccer(|r| r.set_cc3np(value)),
                3 => tim.with_ccer(|r| r.set_cc4np(value)),
                _ => panic!("Only Ch 1..4 supported"),
            }
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

    // TODO: 16 bit devices should use 16 bit values

    pub fn counter(&self) -> u32 {
        let tim = self.tim;
        unsafe {
            tim.cnt().0 as u32
        }
    }

    pub fn set_counter(&self, value: u32) {
        let mut tim = self.tim;
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
        let mut tim = self.tim;
        unsafe {
            tim.set_arr(Arr(value))
        }
    }


    pub fn capture_compare(&self, index: usize) -> u16 {
        let tim = self.tim;
        unsafe {
            match index {
                0 => tim.ccr1().ccr1() as u16,
                1 => tim.ccr2().ccr2() as u16,
                2 => tim.ccr3().ccr3() as u16,
                3 => tim.ccr4().ccr4() as u16,
                4 => tim.ccr5().ccr5() as u16,
                5 => tim.ccr6().ccr6() as u16,
                _ => panic!("Only Ch 1..6 supported"),
            }
        }
    }

    pub fn set_capture_compare(&self, index: usize, value: u16) {
        let mut tim = self.tim;
        unsafe {
            match index {
                0 => tim.set_ccr1(Ccr1(0).set_ccr1(value as u32)),
                1 => tim.set_ccr2(Ccr2(0).set_ccr2(value as u32)),
                2 => tim.set_ccr3(Ccr3(0).set_ccr3(value as u32)),
                3 => tim.set_ccr4(Ccr4(0).set_ccr4(value as u32)),
                4 => tim.set_ccr5(Ccr5(0).set_ccr5(value as u32)),
                5 => tim.set_ccr6(Ccr6(0).set_ccr6(value as u32)),
                _ => panic!("Only Ch 1..6 supported"),
            }
        }
    }

}