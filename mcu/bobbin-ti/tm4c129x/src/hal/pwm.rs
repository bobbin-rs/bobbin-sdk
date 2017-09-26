pub use bobbin_common::timer::*;
pub use chip::pwm::*;
pub use super::sysctl::SysctlEnabled;


    // Enable the PWM clock by setting its corresponding bit in the RCGCPWM register in the System Control module (see page 398).
    // Enable the clock to the appropriate GPIO module via the RCGCGPIO register in the System Control module (see page 382).
    // In the GPIO module, enable the appropriate pins for their alternate function using the GPIOAFSEL register. To determine which GPIOs to configure, see Table 26-4 on page 1797.
    // Configure the PMCn fields in the GPIOPCTL register to assign the PWM signals to the appropriate pins (see page 787 and Table 26-5 on page 1808).
    // Configure the PWM Clock Configuration (PWMCC) register to use the PWM divide (USEPWMDIV) and set the divider (PWMDIV) to divide by 2 (0x0).
    // Configure the PWM generator for countdown mode with immediate updates to the parameters.
    //  - Write the PWM0CTL register with a value of 0x0000.0000.
    //  - Write the PWM0GENA register with a value of 0x0000.008C.
    //  - Write the PWM0GENB register with a value of 0x0000.080C.
    // Set the period. For a 25-KHz frequency, the period = 1/25,000, or 40 microseconds. The PWM clock source is 10 MHz; the system clock divided by 2. Thus there are 400 clock ticks per period. Use this value to set the PWM0LOAD register. In Count-Down mode, set the LOAD field in the PWM0LOAD register to the requested period minus one.
    //  - Write the PWM0LOAD register with a value of 0x0000.018F. 8. Set the pulse width of the MnPWM0 pin for a 25% duty cycle.
    //  - Write the PWM0CMPA register with a value of 0x0000.012B. 9. Set the pulse width of the MnPWM1 pin for a 75% duty cycle.
    //  - Write the PWM0CMPB register with a value of 0x0000.0063. 10. Start the timers in PWM generator 0.
    //  - Write the PWM0CTL register with a value of 0x0000.0001. 11. Enable PWM outputs.
    //  - Write the PWMENABLE register with a value of 0x0000.0003.
impl PwmPeriph {    
    pub fn output_enabled(&self, index: usize) -> bool {
        self.enable().pwmen(index) != 0
    }

    pub fn set_output_enabled(&self, index: usize, value: bool) -> &Self {
        self.with_enable(|r| r.set_pwmen(index, value))
    }    
}

impl PwmCh {
    pub fn enabled(&self) -> bool {
        self.periph.ch_ctl(self.index).enable() != 0
    }

    pub fn set_enabled(&self, value: bool) -> &Self {
        self.periph.with_ch_ctl(self.index, |r| r.set_enable(value));
        self
    }
}

impl Stop for PwmCh {
    fn stop(&self) -> &Self {
        self.set_enabled(false)
    }
}

impl Running for PwmCh {
    fn running(&self) -> bool {
        self.enabled()
    }
}

impl Period<u16> for PwmCh {
    fn period(&self) -> u16 {
        self.periph.ch_load(self.index).load().value()
    }
}

impl SetPeriod<u16> for PwmCh {
    fn set_period(&self, value: u16) -> &Self {
        self.periph.set_ch_load(self.index, |r| r.set_load(value));
        self
    }
}

impl Counter<u16> for PwmCh {
    fn counter(&self) -> u16 {
        self.periph.ch_count(self.index).count().value()
    }
}

impl SetCounter<u16> for PwmCh {
    fn set_counter(&self, value: u16) -> &Self {
        self.periph.set_ch_count(self.index, |r| r.set_count(value));
        self
    }    
}

impl Timeout for PwmCh {
    fn test_timeout(&self) -> bool {
        self.periph.ch_ris(self.index).intcntload() != 0
    }

    fn clr_timeout(&self) -> &Self {
        self.periph.set_ch_isc(self.index, |r| r.set_intcntload(1));
        self        
    }
}

impl Compare<u16> for PwmCh {
    fn compare(&self) -> u16 {
        self.periph.ch_cmpa(self.index).cmpa().value()
    }

    fn set_compare(&self, value: u16) -> &Self {
        self.periph.set_ch_cmpa(self.index, |r| r.set_cmpa(value));
        self
    }

    fn test_compare(&self) -> bool {
        self.periph.ch_ris(self.index).intcmpau() != 0
    }

    fn clr_compare(&self) -> &Self {
        self.periph.set_ch_isc(self.index, |r| r.set_intcmpau(1));
        self
    }
}

impl PwmDownHigh<u16> for PwmCh {
    // Down Counting PWM, (Counter < Compare) => Output High
    fn pwm_down_high(&self, compare: u16, period: u16) -> &Self {
        self.periph.with_ch_gena(self.index, |r| r .set_actload(0x2).set_actcmpad(0x3));
        self
            .set_compare(compare)
            .set_period(period)
            .set_enabled(true)
    }
}

impl PwmDownLow<u16> for PwmCh {
    // Down Counting PWM, (Counter < Compare) => Output Low
    fn pwm_down_low(&self, compare: u16, period: u16) -> &Self {
        self.periph.with_ch_gena(self.index, |r| r .set_actload(0x3).set_actcmpad(0x2));
        self
            .set_compare(compare)
            .set_period(period)
            .set_enabled(true)
    }
}