pub use bobbin_common::timer::*;
pub use chip::lptim::*;

pub trait LptimExt {    
    fn enabled(&self) -> bool;
    fn set_enabled(&self, value: bool) -> &Self;
}

impl<T> LptimExt for Periph<T> {
    fn enabled(&self) -> bool {
        self.cr().enable() != 0
    }
    fn set_enabled(&self, value: bool) -> &Self {
        self.with_cr(|r| r.set_enable(value as u32))
    }
}


impl<T> Timer<u16> for Periph<T> {
    fn start(&self, value: u16) -> &Self {
        self
            .set_period(value)
            .with_cr(|r| r.set_cntstrt(1))
    }

    fn stop(&self) -> &Self {
        self.set_enabled(false)
    }

    fn running(&self) -> bool {
        self.enabled()
    }

    fn period(&self) -> u16 {
        (self.arr().arr() + 1) as u16
    }
    
    fn set_period(&self, value: u16) -> &Self {
        self
            .set_enabled(true)
            .set_arr(Arr(0).set_arr((value - 1) as u32));
        while self.isr().arrok() == 0 {}            
        self
    }

    fn counter(&self) -> u16 {
        self.cnt().cnt() as u16
    }

    fn timeout_flag(&self) -> bool {
        self.isr().arrm() != 0
    }
    fn clr_timeout_flag(&self) -> &Self {
        self.set_icr(Icr(0).set_arrmcf(1))
    }
}

impl<T> Prescale<u16> for Periph<T> {
    fn prescale(&self) -> u16 {
        1 << self.cfgr().presc()
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

impl<T> Compare<u16> for Periph<T> {
    fn compare(&self) -> u16 {
        self.cmp().cmp() as u16
    }
    fn set_compare(&self, value: u16) -> &Self {
        self
            .set_enabled(true)
            .set_cmp(Cmp(0).set_cmp(value as u32));
        while self.isr().cmpok() == 0 {}
        self
        
    }

    fn compare_flag(&self) -> bool {
        self.isr().cmpm() != 0
    }
    fn clr_compare_flag(&self) -> &Self {
        self.set_icr(Icr(0).set_cmpmcf(1))
    }
}