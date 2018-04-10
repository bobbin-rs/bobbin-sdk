pub use periph::tim_adv::*;
use bobbin_common::bits::*;

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

impl From<CcSelect> for U2 {
    fn from(other: CcSelect) -> U2 {
        match other {
            CcSelect::Output => U2::B00,
            CcSelect::InputA => U2::B01,
            CcSelect::InputB => U2::B10,
            CcSelect::InputTRC => U2::B11,
        }
    }
}

impl TimAdvPeriph {
    pub fn set_enabled(&self, value: bool) -> &Self {
        self.with_cr1(|r| r.set_cen(value))
    }

    pub fn set_direction(&self, value: Direction) -> &Self {
        self.with_cr1(|r| r.set_dir(value as u32))
    }

    pub fn set_prescaler(&self, value: u16) -> &Self {
        self.set_psc(|r| r.set_psc(value as u32))
    }

    pub fn set_update_event(&self) -> &Self {
        self.set_egr(|r| r.set_ug(1))
    }

    pub fn update_interrupt_flag(&self) -> bool {
        self.sr().uif() != 0
    }

    pub fn clr_update_interrupt_flag(&self) -> &Self {
        self.with_sr(|r| r.set_uif(0))
    }    

    pub fn set_auto_reload(&self, value: u16) -> &Self {
        self.set_arr(|r| r.set_arr(value))
    }

    pub fn auto_reload(&self) -> u16 {
        self.arr().arr().value()
    }

    pub fn counter(&self) -> u32 {
        self.cnt().cnt().into()
    }

    pub fn set_counter(&self, value: u32) -> &Self {
        self.set_cnt(|r| r.set_cnt(value))
    }
    

    pub fn delay(&self, reload: u16, prescaler: u16) {
        self
            .set_prescaler(prescaler)
            .set_update_event()
            .clr_update_interrupt_flag()
            .set_auto_reload(reload)
            .set_enabled(true);
        while self.update_interrupt_flag() == false {}
        self
            .clr_update_interrupt_flag()
            .set_enabled(false);
    }    

    pub fn set_output_compare_preload_enabled(&self, index: usize, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        match index {
            0...1 => self.with_ccmr_output(0, |r| r.set_ocpe(index, value as u32)),
            2...3 => self.with_ccmr_output(1, |r| r.set_ocpe(index - 2, value as u32)),
            _ => panic!("Invalid channel index"),
        }
    }    

    pub fn set_output_compare_mode(&self, index: usize, value: OcMode) -> &Self {
        let value = value as u32;
        let v012 = value & 0b111;
        let v3 = value >> 3;
        match index {
            0...1 => self.with_ccmr_output(0, |r| r.set_ocm(index, v012).set_occe(index, v3)),
            2...3 => self.with_ccmr_output(1, |r| r.set_ocm(index - 2, v012).set_occe(index - 2, v3)),
            _ => panic!("Invalid channel index"),
        }
    }    

    pub fn set_preload_enable(&self, index: usize, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        match index {
            0...1 => self.with_ccmr_output(0, |r| r.set_ocpe(index, value)),
            2...3 => self.with_ccmr_output(1, |r| r.set_ocpe(index - 2, value)),
            _ => panic!("Invalid channel index"),
        }
    }       

    pub fn set_capture_compare_selection(&self, index: usize, value: CcSelect) -> &Self {
        let value: U2 = value.into();
        match index {
            0...1 => self.with_ccmr_output(0, |r| r.set_ccs(index, value)),
            2...3 => self.with_ccmr_output(1, |r| r.set_ccs(index - 2, value)),
            _ => panic!("Invalid channel index"),
        }
    }    

    pub fn set_capture_compare_enabled(&self, index: usize, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.with_ccer(|r| r.set_cce(index, value))
    }

    pub fn set_capture_compare(&self, index: usize, value: u16) -> &Self {
        self.set_ccr(index, |r| r.set_ccr(value))
    }    

    pub fn capture_compare(&self, index: usize) -> u16 {
        self.ccr(index).ccr().value()
    }

    pub fn output_enable(&self, index: usize) -> bool {
        self.ccer().test_cce(index)
    }

    pub fn set_output_enable(&self, index: usize, value: bool) -> &Self {
        self.with_ccer(|r| r.set_cce(index, value))
    }

    pub fn set_main_output_enable(&self, value: bool) -> &Self {
        self.with_bdtr(|r| r.set_moe(value))
    }


}

impl TimAdvCh {
    pub fn set_preload_enable(&self, value: bool) -> &Self {
        self.periph.set_preload_enable(self.index, value);
        self        
    }
    pub fn set_output_compare_mode(&self, value: OcMode) -> &Self {
        self.periph.set_output_compare_mode(self.index, value);
        self
    }
    pub fn set_capture_compare_selection(&self, value: CcSelect) -> &Self {
        self.periph.set_capture_compare_selection(self.index, value);
        self
    }    
    pub fn set_capture_compare_enabled(&self, value: bool) -> &Self {
        self.periph.set_capture_compare_enabled(self.index, value);
        self
    }
    pub fn set_capture_compare(&self, value: u16) -> &Self {
        self.periph.set_capture_compare(self.index, value);
        self
    }
    pub fn capture_compare(&self) -> u16 {
        self.periph.capture_compare(self.index)
    }
    pub fn output_enable(&self) -> bool {
        self.periph.output_enable(self.index)
    }
    pub fn set_output_enable(&self, value: bool) -> &Self {
        self.periph.set_output_enable(self.index, value);
        self
    }
}