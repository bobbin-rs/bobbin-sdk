pub use bobbin_hal::timer::*;
pub use chip::tcc::*;
pub use super::pm::PmEnabled;

// Review for register synchronization

pub trait TccExt {
    fn mode_up(&self) -> &Self;
    fn mode_down(&self) -> &Self;
    fn mode_continuous(&self) -> &Self;
    fn mode_oneshot(&self) -> &Self;
}

impl TccExt for TccPeriph {
    fn mode_up(&self) -> &Self {
        self.set_ctrlbclr(|r| r.set_dir(1))
    }

    fn mode_down(&self) -> &Self {
        self.set_ctrlbset(|r| r.set_dir(1))
    }

    fn mode_continuous(&self) -> &Self {
        self.set_ctrlbset(|r| r.set_oneshot(1))
    }

    fn mode_oneshot(&self) -> &Self {
        self.set_ctrlbset(|r| r.set_oneshot(1))
    }        
}

impl TccCh {
    pub fn cc(&self) -> u32 {
        self.periph.cc(self.index).cc().value()
    }
    pub fn set_cc(&self, value: u32) -> &Self {
        self.periph.with_cc(self.index, |r| r.set_cc(value));
        self
    }
}

impl Stop for TccPeriph {
    fn stop(&self) -> &Self {
        self.set_ctrlbset(|r| r.set_cmd(0x2));
        self.with_ctrla(|r| r.set_enable(0x0));
        self
    }
}

impl Running for TccPeriph {
    fn running(&self) -> bool {
        self.status().stop() == 0
    }
}

impl Period<u16> for TccPeriph {
    fn period(&self) -> u16 {
        // Discard excess bits
        self.per().per().value() as u16
    }
}

impl SetPeriod<u16> for TccPeriph {
    fn set_period(&self, value: u16) -> &Self {
        self.set_per(|r| r.set_per(value));
        while self.syncbusy().per() != 0 {}
        self
    }
}
impl Counter<u16> for TccPeriph {
    fn counter(&self) -> u16 {
        // Discard excess bits
        self.count().count().value() as u16
    }
}

impl Timeout for TccPeriph {
    fn test_timeout(&self) -> bool {
        self.intflag().ovf() != 0
    }

    fn clr_timeout(&self) -> &Self {
        self.set_intflag(|r| r.set_ovf(1));
        self
    }    
}

impl Start<u16> for TccPeriph {
    fn start(&self, value: u16) -> &Self {        
        self.set_period(value);        
        self.with_wave(|r| r.set_wavegen(0x0));
        self.with_ctrla(|r| r.set_enable(0x1));
        self.set_ctrlbset(|r| r.set_cmd(0x1));
        self
    }
}

impl StartUp<u16> for TccPeriph {
    fn start_up(&self, value: u16) -> &Self {        
        self
            .set_period(value)
            .with_wave(|r| r.set_wavegen(0x0))
            .mode_up()
            .mode_continuous()
            .with_ctrla(|r| r.set_enable(0x1))
            .set_ctrlbset(|r| r.set_cmd(0x1))
    }
}


impl StartDown<u16> for TccPeriph {
    fn start_down(&self, value: u16) -> &Self {        
        self
            .set_period(value)
            .with_wave(|r| r.set_wavegen(0x0))
            .mode_down()
            .mode_continuous()
            .with_ctrla(|r| r.set_enable(0x1))
            .set_ctrlbset(|r| r.set_cmd(0x1))
    }
}


impl StartUpOnce<u16> for TccPeriph {
    fn start_up_once(&self, value: u16) -> &Self {        
        self
            .set_period(value)
            .with_wave(|r| r.set_wavegen(0x0))
            .mode_up()
            .mode_oneshot()
            .with_ctrla(|r| r.set_enable(0x1))
            .set_ctrlbset(|r| r.set_cmd(0x1))
    }
}


impl StartDownOnce<u16> for TccPeriph {
    fn start_down_once(&self, value: u16) -> &Self {        
        self
            .set_period(value)
            .with_wave(|r| r.set_wavegen(0x0))
            .mode_down()
            .mode_oneshot()
            .with_ctrla(|r| r.set_enable(0x1))
            .set_ctrlbset(|r| r.set_cmd(0x1))
    }
}


impl Prescale<u16> for TccPeriph {
    fn prescale(&self) -> u16 {
        1 << self.ctrla().prescaler().value() 
    }
}

impl SetPrescale<u16> for TccPeriph {
    fn set_prescale(&self, value: u16) -> &Self {
        let shift = match value {
            1 => 0,
            2 => 1,
            4 => 2,
            8 => 3,
            16 => 4,
            32 => 5,
            64 => 6,
            128 => 7,
            256 => 8,
            512 => 9,
            1024 => 10,
            _ => panic!("Invalid prescaler value"),
        };
        self.with_ctrla(|r| r.set_prescaler(shift))
    }
}

impl SetCounter<u16> for TccPeriph {
    fn set_counter(&self, value: u16) -> &Self {
        self.set_count(|r| r.set_count(value));
        self
    }
}


impl Compare<u16> for TccCh {
    fn compare(&self) -> u16 {
        // Discard Excess Bits
        self.periph.cc(self.index).cc().value() as u16
    }

    fn set_compare(&self, value: u16) -> &Self {
        self.periph.set_cc(self.index, |r| r.set_cc(value));
        self
    }

    fn test_compare(&self) -> bool {
        self.periph.intflag().mc(self.index) != 0
    }

    fn clr_compare(&self) -> &Self {
        self.periph.set_intflag(|r| r.set_mc(self.index, 1));
        self
    }

}

impl PwmUpLow<u16> for TccCh {
    // Up Counting PWM, (Counter < Compare) => Output Low
    fn pwm_up_low(&self, compare: u16, period: u16) -> &Self {
        self.set_compare(compare);
        self.periph
            .set_period(period)
            .with_wave(|r| r.set_pol(self.index, 0).set_wavegen(0x02))
            .mode_up()
            .with_ctrla(|r| r.set_enable(0x1));
        self
    }
}

impl PwmDownLow<u16> for TccCh {
    // Down Counting PWM, (Counter < Compare) => Output Low
    fn pwm_down_low(&self, compare: u16, period: u16) -> &Self {
        self.set_compare(compare);
        self.periph
            .set_period(period)
            .with_wave(|r| r.set_pol(self.index, 0).set_wavegen(0x02))
            .mode_down()
            .with_ctrla(|r| r.set_enable(0x1));
        self
    }
}

impl PwmUpHigh<u16> for TccCh {
    // Up Counting PWM, (Counter < Compare) => Output High
    fn pwm_up_high(&self, compare: u16, period: u16) -> &Self {
        self.set_compare(compare);
        self.periph
            .set_period(period)
            .with_wave(|r| r.set_pol(self.index, 1).set_wavegen(0x02))            
            .mode_up()
            .with_ctrla(|r| r.set_enable(0x1));
        self
    }
}


impl PwmDownHigh<u16> for TccCh {
    // Down Counting PWM, (Counter < Compare) => Output High
    fn pwm_down_high(&self, compare: u16, period: u16) -> &Self {
        self.set_compare(compare);
        self.periph
            .set_period(period)
            .with_wave(|r| r.set_pol(self.index, 1).set_wavegen(0x02))
            .mode_down()
            .with_ctrla(|r| r.set_enable(0x1));
        self
    }
}