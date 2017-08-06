pub use chip::tim_adv::*;

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

pub trait TimAdvExt {
    fn set_enabled(&self, value: bool) -> &Self;
    fn set_direction(&self, value: Direction) -> &Self;
    fn set_prescaler(&self, value: u16) -> &Self;
    fn set_update_event(&self) -> &Self;
    fn update_interrupt_flag(&self) -> bool;
    fn clr_update_interrupt_flag(&self) -> &Self;
    fn set_auto_reload(&self, value: u32) -> &Self;
    fn counter(&self) -> u32;
    fn set_counter(&self, value: u32) -> &Self;
    fn delay(&self, reload: u32, prescaler: u16);
    fn set_output_compare_preload_enabled(&self, index: usize, value: bool) -> &Self;
    fn set_output_compare_mode(&self, index: usize, value: OcMode) -> &Self;
    fn set_capture_compare_enabled(&self, index: usize, value: bool) -> &Self;
    fn set_capture_compare(&self, index: usize, value: u32) -> &Self;
    fn set_preload_enable(&self, index: usize, value: bool) -> &Self;
}

impl<T> TimAdvExt for Periph<T> {
    fn set_enabled(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.with_cr1(|r| r.set_cen(value))
    }

    fn set_direction(&self, value: Direction) -> &Self {
        self.with_cr1(|r| r.set_dir(value as u32))
    }

    fn set_prescaler(&self, value: u16) -> &Self {
        self.set_psc(Psc(0).set_psc(value as u32))
    }

    fn set_update_event(&self) -> &Self {
        self.set_egr(Egr(0).set_ug(1))
    }

    fn update_interrupt_flag(&self) -> bool {
        self.sr().uif() != 0
    }

    fn clr_update_interrupt_flag(&self) -> &Self {
        self.with_sr(|r| r.set_uif(0))
    }    

    fn set_auto_reload(&self, value: u32) -> &Self {
        self.set_arr(Arr(value))
    }

    fn counter(&self) -> u32 {
        self.cnt().cnt().into()
    }

    fn set_counter(&self, value: u32) -> &Self {
        self.set_cnt(Cnt(value))
    }
    

    fn delay(&self, reload: u32, prescaler: u16) {
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

    fn set_output_compare_preload_enabled(&self, index: usize, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        match index {
            0...1 => self.with_ccmr_output(0, |r| r.set_ocpe(index, value as u32)),
            2...3 => self.with_ccmr_output(1, |r| r.set_ocpe(index - 2, value as u32)),
            _ => panic!("Invalid channel index"),
        }
    }    

    fn set_output_compare_mode(&self, index: usize, value: OcMode) -> &Self {
        let value = value as u32;
        let v012 = value & 0b111;
        let v3 = value >> 3;
        match index {
            0...1 => self.with_ccmr_output(0, |r| r.set_ocm(index, v012).set_occe(index, v3)),
            2...3 => self.with_ccmr_output(1, |r| r.set_ocm(index - 2, v012).set_occe(index - 2, v3)),
            _ => panic!("Invalid channel index"),
        }
    }    

    fn set_preload_enable(&self, index: usize, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        match index {
            0...1 => self.with_ccmr_output(0, |r| r.set_ocpe(index, value)),
            2...3 => self.with_ccmr_output(1, |r| r.set_ocpe(index - 2, value)),
            _ => panic!("Invalid channel index"),
        }
    }       

    fn set_capture_compare_enabled(&self, index: usize, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.with_ccer(|r| r.set_cce(index, value))
    }

    fn set_capture_compare(&self, index: usize, value: u32) -> &Self {
        self.set_ccr(index, Ccr(value))
    }    

}

pub trait TimGenChExt {
    fn set_preload_enable(&self, bool) -> &Self;
    fn set_output_compare_mode(&self, value: OcMode) -> &Self;
    fn set_capture_compare_enabled(&self, value: bool) -> &Self;
    fn set_capture_compare(&self, value: u32) -> &Self;
}

impl<P, T> TimGenChExt for Channel<P, T> {
    fn set_preload_enable(&self, value: bool) -> &Self {
        self.periph().set_preload_enable(self.index(), value);
        self        
    }
    fn set_output_compare_mode(&self, value: OcMode) -> &Self {
        self.periph().set_output_compare_mode(self.index(), value);
        self
    }
    fn set_capture_compare_enabled(&self, value: bool) -> &Self {
        self.periph().set_capture_compare_enabled(self.index(), value);
        self
    }
    fn set_capture_compare(&self, value: u32) -> &Self {
        self.periph().set_capture_compare(self.index(), value);
        self
    }
}