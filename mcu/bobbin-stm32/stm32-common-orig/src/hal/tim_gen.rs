pub use bobbin_hal::timer::*;
pub use ::chip::tim_gen::*;

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

impl TimGenPeriph {
    pub fn set_enabled(&self, value: bool) -> &Self {
        self.with_cr1(|r| r.set_cen(value))
    }

    pub fn set_direction(&self, value: Direction) -> &Self {
        self.with_cr1(|r| r.set_dir(value as u32))
    }

    // pub fn set_prescaler(&self, value: u16) -> &Self {
    //     self.set_psc(Psc(0).set_psc(value))
    // }

    pub fn set_update_event(&self) -> &Self {
        self.set_egr(|r| r.set_ug(1))
    }

    pub fn update_interrupt_flag(&self) -> bool {
        self.sr().uif() != 0
    }

    pub fn clr_update_interrupt_flag(&self) -> &Self {
        self.with_sr(|r| r.set_uif(0))
    }    

    pub fn set_auto_reload(&self, value: u32) -> &Self {
        self.set_arr(|r| r.set_arrl(value))
    }

    // pub fn delay(&self, reload: u32, prescaler: u16) {
    //     self
    //         .set_prescaler(prescaler)
    //         .set_update_event()
    //         .clr_update_interrupt_flag()
    //         .set_auto_reload(reload)
    //         .set_enabled(true);
    //     while self.update_interrupt_flag() == false {}
    //     self
    //         .clr_update_interrupt_flag()
    //         .set_enabled(false);
    // }    

    pub fn set_output_compare_preload_enabled(&self, index: usize, value: bool) -> &Self {
        match index {
            0...1 => self.with_ccmr_output(0, |r| r.set_ocpe(index, value)),
            2...3 => self.with_ccmr_output(1, |r| r.set_ocpe(index - 2, value)),
            _ => panic!("Invalid channel index"),
        }
    }    

    pub fn set_output_compare_mode(&self, index: usize, value: OcMode) -> &Self {
        let value = value as u32;
        let v012 = value & 0b111;
        let v3 = value >> 3;
        match index {
            0...1 => self.with_ccmr_output(0, |r| r.set_ocm(index, v012).set_ocm_3(index, v3)),
            2...3 => self.with_ccmr_output(1, |r| r.set_ocm(index - 2, v012).set_ocm_3(index - 2, v3)),
            _ => panic!("Invalid channel index"),
        }
    }    

    pub fn set_capture_compare_enabled(&self, index: usize, value: bool) -> &Self {
        self.with_ccer(|r| r.set_cce(index, value))
    }

    pub fn capture_compare(&self, index: usize) -> u32 {
        self.ccr(index).0
    }

    pub fn set_capture_compare(&self, index: usize, value: u32) -> &Self {
        self.set_ccr(index, |r| r.set_ccrl(value))
    }    
}

impl TimGenCh {
    pub fn set_output_compare_mode(&self, value: OcMode) -> &Self {
        self.periph.set_output_compare_mode(self.index, value);
        self
    }
    pub fn set_capture_compare_enabled(&self, value: bool) -> &Self {
        self.periph.set_capture_compare_enabled(self.index, value);
        self
    }

    pub fn capture_compare(&self) -> u32 {
        self.periph.capture_compare(self.index)
    }

    pub fn set_capture_compare(&self, value: u32) -> &Self {
        self.periph.set_capture_compare(self.index, value);
        self
    }
}

impl SetCounter<u16> for TimGenPeriph {
    fn set_counter(&self, value: u16) -> &Self {
        self.set_cnt(|r| r.set_cntl(value))
    }    
}

impl Prescale<u16> for TimGenPeriph {
    fn prescale(&self) -> u16 {
        self.psc().psc().value() + 1
    }
}

impl SetPrescale<u16> for TimGenPeriph {
    fn set_prescale(&self, value: u16) -> &Self {
        self
            .set_psc(|r| r.set_psc(value - 1))
            .set_egr(|r| r.set_ug(1))
    }    
}

impl Start<u16> for TimGenPeriph {
    fn start(&self, value: u16) -> &Self {
        self.start_up(value)
    }
}

impl StartOnce<u16> for TimGenPeriph {
    fn start_once(&self, value: u16) -> &Self {
        self.start_up_once(value)
    }
}

impl StartUp<u16> for TimGenPeriph {
    fn start_up(&self, value: u16) -> &Self {
        self
            .set_arr(|r| r.set_arrl(value - 1))
            .set_egr(|r| r.set_ug(1))
            .with_cr1(|r| r.set_dir(0).set_opm(0).set_cen(1))
    }
}

impl StartUpOnce<u16> for TimGenPeriph {
    fn start_up_once(&self, value: u16) -> &Self {
        self
            .set_arr(|r| r.set_arrl(value - 1))
            .set_egr(|r| r.set_ug(1))
            .with_cr1(|r| r.set_dir(0).set_opm(1).set_cen(1))
    }
}

impl StartDown<u16> for TimGenPeriph {
    fn start_down(&self, value: u16) -> &Self {
        self
            .set_arr(|r| r.set_arrl(value - 1))
            .set_egr(|r| r.set_ug(1))
            .with_cr1(|r| r.set_dir(1).set_opm(0).set_cen(1))
    }
}

impl StartDownOnce<u16> for TimGenPeriph {
    fn start_down_once(&self, value: u16) -> &Self {
        self
            .set_arr(|r| r.set_arrl(value - 1))
            .set_egr(|r| r.set_ug(1))
            .with_cr1(|r| r.set_dir(1).set_opm(1).set_cen(1))
    }
}

impl Delay<u16> for TimGenPeriph {
    fn delay(&self, value: u16) -> &Self {
        self
            .start_once(value)
            .clr_timeout()
            .wait_timeout()
    }
}

impl Stop for TimGenPeriph {
    fn stop(&self) -> &Self {
        self.with_cr1(|r| r.set_cen(0))
    }
}

impl Running for TimGenPeriph {
    fn running(&self) -> bool {
        self.cr1().cen() != 0
    }
}

impl Period<u16> for TimGenPeriph {
    fn period(&self) -> u16 {
        self.arr().arrl().value() + 1
    }
}

impl SetPeriod<u16> for TimGenPeriph {  
    fn set_period(&self, value: u16) -> &Self {
        self.set_arr(|r| r.set_arrl(value - 1 ))
    }
}

impl Counter<u16> for TimGenPeriph {
    fn counter(&self) -> u16 {
        self.cnt().cntl().value()
    }
}

impl Timeout for TimGenPeriph {
    fn test_timeout(&self) -> bool {
        self.sr().uif() != 0
    }

    fn clr_timeout(&self) -> &Self {
        self.with_sr(|r| r.set_uif(0))
    }
}

impl Compare<u16> for TimGenCh {
    fn compare(&self) -> u16 {
        self.periph.ccr(self.index).ccrl().value()
    }
    fn set_compare(&self, value: u16) -> &Self {
        self.periph
            .set_ccr(self.index, |r| r.set_ccrl(value))
            .set_egr(|r| r.set_ug(1));
        self
    }

    fn test_compare(&self) -> bool {
        self.periph.sr().ccif(self.index) != 0
    }
    fn clr_compare(&self) -> &Self {
        self.periph.with_sr(|r| r.set_ccif(self.index, 0));
        self    
    }
}

impl PwmUpHigh<u16> for TimGenCh {    
    // Up Counting PWM, (Counter < Compare) => Output High
    fn pwm_up_high(&self, compare: u16, period: u16) -> &Self {
        self
            .set_output_compare_mode(OcMode::Pwm1)
            .set_capture_compare_enabled(true)
            .set_compare(compare);
        self.periph.start_up(period);
        self
    }
}

impl PwmUpLow<u16> for TimGenCh {    
    // Up Counting PWM, (Counter < Compare) => Output High
    fn pwm_up_low(&self, compare: u16, period: u16) -> &Self {
        self
            .set_output_compare_mode(OcMode::Pwm2)
            .set_capture_compare_enabled(true)
            .set_compare(compare);
        self.periph.start_up(period);
        self
    }
}

impl PwmDownHigh<u16> for TimGenCh {    
    // Down Counting PWM, (Counter < Compare) => Output High
    fn pwm_down_high(&self, compare: u16, period: u16) -> &Self {
        self
            .set_output_compare_mode(OcMode::Pwm1)
            .set_capture_compare_enabled(true)
            .set_compare(compare);
        self.periph.start_down(period);
        self
    }
}

impl PwmDownLow<u16> for TimGenCh {    
    // Down Counting PWM, (Counter < Compare) => Output High
    fn pwm_down_low(&self, compare: u16, period: u16) -> &Self {
        self
            .set_output_compare_mode(OcMode::Pwm2)
            .set_capture_compare_enabled(true)
            .set_compare(compare);
        self.periph.start_down(period);
        self
    }
}