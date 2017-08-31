pub use bobbin_common::timer::*;
pub use chip::lptim::*;

impl LptimPeriph {
    pub fn enabled(&self) -> bool {
        self.cr().enable() != 0
    }
    pub fn set_enabled(&self, value: bool) -> &Self {
        self.with_cr(|r| r.set_enable(value as u32))
    }
}

impl Start<u16> for LptimPeriph {
    fn start(&self, value: u16) -> &Self {
        self.start_up(value)
    }
}

impl StartUp<u16> for LptimPeriph {
    fn start_up(&self, value: u16) -> &Self {
        self
            .set_period(value)
            .with_cr(|r| r.set_cntstrt(1))
    }
}

impl StartUpOnce<u16> for LptimPeriph {
    fn start_up_once(&self, value: u16) -> &Self {
        self
            .set_period(value)
            .with_cr(|r| r.set_sngstrt(1))
    }
}

impl Delay<u16> for LptimPeriph {
    fn delay(&self, value: u16) -> &Self {
        self
            .start_up_once(value)
            .clr_timeout_flag()
            .wait_timeout_flag()
            .stop()
    }
}

impl Timer<u16> for LptimPeriph {
    fn stop(&self) -> &Self {
        self.set_enabled(false)
    }

    fn running(&self) -> bool {
        self.enabled()
    }

    fn period(&self) -> u16 {
        self.arr().arr().value() + 1
    }
    
    fn set_period(&self, value: u16) -> &Self {
        self
            .set_enabled(true)
            .set_arr(|r| r.set_arr((value - 1)));
        while self.isr().arrok() == 0 {}            
        self
    }

    fn counter(&self) -> u16 {
        self.cnt().cnt().value()
    }

    fn timeout_flag(&self) -> bool {
        self.isr().arrm() != 0
    }

    fn clr_timeout_flag(&self) -> &Self {
        self.set_icr(|r| r.set_arrmcf(1))
    }
}

impl Prescale<u16> for LptimPeriph {
    fn prescale(&self) -> u16 {
        1 << self.cfgr().presc().value()
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
            _ => panic!("Unsupported prescaler value"),
        };
        self.with_cfgr(|r| r.set_presc(shift))
    }
}

impl Compare<u16> for LptimPeriph {
    fn compare(&self) -> u16 {
        self.cmp().cmp().value()
    }

    fn set_compare(&self, value: u16) -> &Self {
        self
            .set_enabled(true)
            .set_cmp(|r| r.set_cmp(value));
        while self.isr().cmpok() == 0 {}
        self
        
    }

    fn compare_flag(&self) -> bool {
        self.isr().cmpm() != 0
    }

    fn clr_compare_flag(&self) -> &Self {
        self.set_icr(|r| r.set_cmpmcf(1))
    }
}