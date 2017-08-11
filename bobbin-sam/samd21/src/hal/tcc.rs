pub use bobbin_common::timer::*;
pub use chip::tcc::*;
pub use super::pm::PmEnabled;

// Review for register synchronization

pub trait TccExt {
    fn mode_up(&self) -> &Self;
    fn mode_down(&self) -> &Self;
    fn mode_continuous(&self) -> &Self;
    fn mode_oneshot(&self) -> &Self;
}

impl<T> TccExt for Periph<T> {
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

pub trait TccChExt {
    fn cc(&self) -> u32;
    fn set_cc(&self, u32) -> &Self;
}

impl<P, T> TccChExt for Channel<P, T> {
    fn cc(&self) -> u32 {
        self.periph.cc(self.index).cc().value()
    }
    fn set_cc(&self, value: u32) -> &Self {
        self.periph.with_cc(self.index, |r| r.set_cc(value));
        self
    }
}

impl<T> Timer<u16> for Periph<T> {
    fn stop(&self) -> &Self {
        self.set_ctrlbset(|r| r.set_cmd(0x2));
        self.with_ctrla(|r| r.set_enable(0x0));
        self
    }
    fn running(&self) -> bool {
        self.status().stop() == 0
    }

    fn period(&self) -> u16 {
        // Discard excess bits
        self.per().per().value() as u16
    }

    fn set_period(&self, value: u16) -> &Self {
        self.set_per(|r| r.set_per(value));
        while self.syncbusy().per() != 0 {}
        self
    }

    fn counter(&self) -> u16 {
        // Discard excess bits
        self.count().count().value() as u16
    }

    fn timeout_flag(&self) -> bool {
        self.intflag().ovf() != 0
    }

    fn clr_timeout_flag(&self) -> &Self {
        self.set_intflag(|r| r.set_ovf(1));
        self
    }    
}

impl<T> Start<u16> for Periph<T> {
    fn start(&self, value: u16) -> &Self {        
        self.set_period(value);        
        self.with_wave(|r| r.set_wavegen(0x0));
        self.with_ctrla(|r| r.set_enable(0x1));
        self.set_ctrlbset(|r| r.set_cmd(0x1));
        self
    }
}

impl<T> StartUp<u16> for Periph<T> {
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


impl<T> StartDown<u16> for Periph<T> {
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


impl<T> StartUpOnce<u16> for Periph<T> {
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


impl<T> StartDownOnce<u16> for Periph<T> {
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


impl<T> Prescale<u16> for Periph<T> {
    fn prescale(&self) -> u16 {
        1 << self.ctrla().prescaler().value() 
    }
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

impl<T> SetCounter<u16> for Periph<T> {
    fn set_counter(&self, value: u16) -> &Self {
        self.set_count(|r| r.set_count(value));
        self
    }
}


impl<P, T> Compare<u16> for Channel<P, T> {
    fn compare(&self) -> u16 {
        // Discard Excess Bits
        self.periph().cc(self.index()).cc().value() as u16
    }

    fn set_compare(&self, value: u16) -> &Self {
        self.periph().set_cc(self.index(), |r| r.set_cc(value));
        self
    }

    fn compare_flag(&self) -> bool {
        self.periph().intflag().mc(self.index()) != 0
    }

    fn clr_compare_flag(&self) -> &Self {
        self.periph().set_intflag(|r| r.set_mc(self.index(), 1));
        self
    }

}

impl<P, T> PwmUpLow<u16> for Channel<P, T> {
    // Up Counting PWM, (Counter < Compare) => Output Low
    fn pwm_up_low(&self, compare: u16, period: u16) -> &Self {
        self.set_compare(compare);
        self.periph()
            .set_period(period)
            .with_wave(|r| r.set_pol(self.index(), 0).set_wavegen(0x02))
            .mode_up()
            .with_ctrla(|r| r.set_enable(0x1));
        self
    }
}

impl<P, T> PwmDownLow<u16> for Channel<P, T> {
    // Down Counting PWM, (Counter < Compare) => Output Low
    fn pwm_down_low(&self, compare: u16, period: u16) -> &Self {
        self.set_compare(compare);
        self.periph()
            .set_period(period)
            .with_wave(|r| r.set_pol(self.index(), 0).set_wavegen(0x02))
            .mode_down()
            .with_ctrla(|r| r.set_enable(0x1));
        self
    }
}

impl<P, T> PwmUpHigh<u16> for Channel<P, T> {
    // Up Counting PWM, (Counter < Compare) => Output High
    fn pwm_up_high(&self, compare: u16, period: u16) -> &Self {
        self.set_compare(compare);
        self.periph()
            .set_period(period)
            .with_wave(|r| r.set_pol(self.index(), 1).set_wavegen(0x02))            
            .mode_up()
            .with_ctrla(|r| r.set_enable(0x1));
        self
    }
}


impl<P, T> PwmDownHigh<u16> for Channel<P, T> {
    // Down Counting PWM, (Counter < Compare) => Output High
    fn pwm_down_high(&self, compare: u16, period: u16) -> &Self {
        self.set_compare(compare);
        self.periph()
            .set_period(period)
            .with_wave(|r| r.set_pol(self.index(), 1).set_wavegen(0x02))
            .mode_down()
            .with_ctrla(|r| r.set_enable(0x1));
        self
    }
}