pub use bobbin_hal::timer::*;
pub use chip::ftm::*;
pub use core::ops::Deref;

pub enum FtmPrescale {
    Div1 = 0b000,
    Div2 = 0b001,
    Div4 = 0b010,
    Div8 = 0b011,
    Div16 = 0b100,
    Div32 = 0b101,
    Div64 = 0b110,
    Div128 = 0b111,
}

pub enum ClockSource {
    Disabled = 0b00,
    SystemClk = 0b01,
    FixedClk = 0b10,
}

impl FtmPeriph {
    pub fn set_clock(&self, value: ClockSource) -> &Self {
        self.with_sc(|r| r.set_clks(value as u32))
    }
    // pub fn set_prescale(&self, value: FtmPrescale) -> &Self {
    //     self.with_sc(|r| r.set_ps(value as u32))
    // }
    pub fn timer_overflow(&self) -> bool {
        self.sc().tof() != 0
    }
    pub fn clr_timer_overflow(&self) -> &Self {
        self.with_sc(|r| r.set_tof(1))    
    }
    pub fn count(&self) -> u16 {
        self.cnt().count().into()
    }
    pub fn set_count(&self, value: u16) -> &Self {
        self.set_cnt(|r| r.set_count(value as u32))
    }
    pub fn modulo(&self) -> u16 {
        self._mod()._mod().into()
    }
    pub fn set_modulo(&self, value: u16) -> &Self {
        self.set_mod(|r| r.set_mod(value as u32))
    }    
    pub fn set_center(&self, value: bool) -> &Self {
        self.with_sc(|r| r.set_cpwms(value))
    }
}

impl FtmCh {
    pub fn csc(&self) -> Csc {
        self.periph.csc(self.index)
    }
    pub fn with_csc<F: FnOnce(Csc) -> Csc>(&self, f: F) -> &Self {
        self.periph.with_csc(self.index, f);
        self
    }
    pub fn value(&self) -> u16 {
        self.periph.cv(self.index).val().value()
    }
    pub fn set_value(&self, value: u16) -> &Self {
        self.periph.set_cv(self.index, |r| r.set_val(value));
        self
    }
    pub fn set_pwmen(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.periph.with_sc(|r| r.set_pwmen(self.index, value));
        self
    }
}

impl Start<u16> for FtmPeriph {
    fn start(&self, value: u16) -> &Self {
        self.start_up(value)
    }
}

impl StartUp<u16> for FtmPeriph {
    fn start_up(&self, value: u16) -> &Self {
        self
            .set_clock(ClockSource::Disabled)
            .set_count(0)
            .set_modulo(value)
            .set_center(false)            
            .set_clock(ClockSource::SystemClk)
            .clr_timeout()            
    }
}

impl Running for FtmPeriph {
    fn running(&self) -> bool {
        self.sc().clks() != 0
    }
}

impl Stop for FtmPeriph {
    fn stop(&self) -> &Self {
        self.set_clock(ClockSource::Disabled)
    }
}

impl Counter<u16> for FtmPeriph {
    fn counter(&self) -> u16 {
        self.cnt().count().value()
    }    
}

impl SetCounter<u16> for FtmPeriph {
    fn set_counter(&self, value: u16) -> &Self {
        self.set_cnt(|r| r.set_count(value))
    }
}


impl Timeout for FtmPeriph {
    fn test_timeout(&self) -> bool {
        self.sc().tof() != 0
    }

    fn clr_timeout(&self) -> &Self {
        let _ = self.sc();
        self.with_sc(|r| r.set_tof(0))
    }        
}

impl Delay<u16> for FtmPeriph {
    fn delay(&self, value: u16) -> &Self {
        self
            .start(value)
            .clr_timeout()
            .wait_timeout()
            .stop()
    }
}

impl Period<u16> for FtmPeriph {
    fn period(&self) -> u16 {
        self.modulo() + 1
    }
}

impl SetPeriod<u16> for FtmPeriph {
    fn set_period(&self, value: u16) -> &Self {
        self.set_modulo(value)
    }
}

impl Prescale<u16> for FtmPeriph {
    fn prescale(&self) -> u16 {
        1u16 << self.sc().ps().value()
    }
}

impl SetPrescale<u16> for FtmPeriph {
    fn set_prescale(&self, value: u16) -> &Self {
        let value = match value {
            1 => 0b000,
            2 => 0b001,
            4 => 0b010,
            8 => 0b011,
            16 => 0b100,
            32 => 0b101,
            64 => 0b110,
            128 => 0b111,
            _ => panic!("Unsupported prescaler value: {}", value),
        };
        self.with_sc(|r| r.set_ps(value))
    }    
}

impl Compare<u16> for FtmCh {
    fn compare(&self) -> u16 {
        self.periph.cv(self.index).val().value()
    }
    fn set_compare(&self, value: u16) -> &Self {
        self.periph.set_cv(self.index, |r| r.set_val(value));
        self
    }

    fn test_compare(&self) -> bool {
        self.periph.csc(self.index).chf() != 0
    }

    fn clr_compare(&self) -> &Self {
        self.periph.with_csc(self.index, |r| r.set_chf(0));
        self    
    }
}

impl PwmLow<u16> for FtmCh {
    // PWM, (Counter < Compare) => Output Low
    fn pwm_low(&self, compare: u16, period: u16) -> &Self {
        self.pwm_up_low(compare, period)
    }
}

impl PwmHigh<u16> for FtmCh {
    // PWM, (Counter < Compare) => Output High
    fn pwm_high(&self, compare: u16, period: u16) -> &Self {
        self.pwm_up_high(compare, period)
    }
}

impl PwmUpLow<u16> for FtmCh {
    // Up Counting PWM, (Counter < Compare) => Output Low
    fn pwm_up_low(&self, compare: u16, period: u16) -> &Self {
        self
            .set_pwmen(true)
            .with_csc(|r| r.set_msb(1).set_msa(0).set_elsb(0).set_elsa(1))
            .set_value(compare);
        self.periph.start(period);
        self
    }
}

impl PwmUpHigh<u16> for FtmCh {
    // Up Counting PWM, (Counter < Compare) => Output High
    fn pwm_up_high(&self, compare: u16, period: u16) -> &Self {
        self
            .set_pwmen(true)
            .with_csc(|r| r.set_msb(1).set_msa(0).set_elsb(1).set_elsa(0))
            .set_value(compare);
        self.periph.start(period);
        self
    }
}

impl PwmCenterLow<u16> for FtmCh {
    // Center Aligned PWM, (Counter < Compare) => Output Low
    fn pwm_center_low(&self, compare: u16, period: u16) -> &Self {
        self
            .set_pwmen(true)
            .with_csc(|r| r.set_msb(1).set_msa(0).set_elsb(0).set_elsa(1))
            .set_value(compare);
        self.periph
            .set_modulo(period - 1)
            .set_center(true)
            .set_clock(ClockSource::SystemClk);
        self
    }
}

impl PwmCenterHigh<u16> for FtmCh {
    // Center Aligned PWM, (Counter < Compare) => Output High
    fn pwm_center_high(&self, compare: u16, period: u16) -> &Self {
        self
            .set_pwmen(true)
            .with_csc(|r| r.set_msb(1).set_msa(0).set_elsb(1).set_elsa(0))
            .set_value(compare);
        self.periph
            .set_modulo(period - 1)
            .set_center(true)
            .set_clock(ClockSource::SystemClk);
        self
    }
}

pub fn test_ftm(ftm: &FtmPeriph) {
    test_timer(ftm, 1024);
    test_timer_up(ftm, 1024);    
}