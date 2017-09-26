pub use bobbin_common::timer::*;
pub use bobbin_common::Channel;
pub use chip::timer::*;
pub use super::sysctl::SysctlEnabled;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Dir {
    Down = 0,
    Up = 1,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Mode {
    OneShot = 1,
    Periodic = 2,
    Capture = 3,
}

// NOTE: Uses timer in Mode 0, 32 bit timer configuration.

impl TimerCh {
    pub fn tmr(&self) -> Tmr {
        self.periph.tmr(self.index)
    }
    pub fn set_tmr(&self, value: Tmr) -> &Self {
        self.periph.set_tmr(self.index, |_| value);
        self
    }
    pub fn with_tmr<F: FnOnce(Tmr) -> Tmr>(&self, f: F) -> &Self {
        self.periph.with_tmr(self.index, f);
        self
    }
    pub fn enabled(&self) -> bool {
        self.periph.ctl().ten(self.index) != 0
    }
    pub fn set_enabled(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.periph.with_ctl(|r| r.set_ten(self.index, value));
        self
    }
    pub fn reload(&self) -> u32 {
        self.periph.tilr(self.index).tilr().value()
    }
    pub fn set_reload(&self, value: u32) -> &Self {
        self.periph.set_tilr(self.index, |_| Tilr(value));
        self
    }

    pub fn prescale(&self) -> u8 {
        self.periph.tpr(self.index).tpsr().value()
    }

    pub fn set_prescale(&self, value: u8) -> &Self {
        self.periph.set_tpr(self.index, |r| r.set_tpsr(value));
        self
    }

    pub fn compare(&self) -> u32 {
        self.periph.tmtchr(self.index).tmtchr().value()
    }
    pub fn set_compare(&self, value: u32) -> &Self {
        self.periph.set_tmtchr(self.index, |_| Tmtchr(value));
        self
    }    
    pub fn value(&self) -> u32 {
        self.periph.tv(self.index).tv().value()
    }
    pub fn set_value(&self, value: u32) -> &Self {
        self.periph.set_tv(self.index, |_| Tv(value));
        self
    }        
    pub fn match_dmaen(&self) -> bool {
        self.periph.dmaev().tmdmaen(self.index) != 0
    }
    pub fn set_match_dmaen(&self, value: bool) -> &Self {
        self.periph.with_dmaev(|r| r.set_tmdmaen(self.index, value));
        self
    }
    #[inline]
    pub fn test_timeout(&self) -> bool {
        self.periph.ris().ttoris(self.index) != 0
    }
    #[inline]
    pub fn clr_timeout(&self) -> &Self {
        self.periph.set_icr(|r| r.set_ttocint(self.index, 1));
        self
    }
    #[inline]
    pub fn match_flag(&self) -> bool {
        match self.index {
            0 => self.periph.ris().tamris() != 0,
            1 => self.periph.ris().tbmris() != 0,            
            _ => panic!("Invalid channel index"),
        }
    }
    #[inline]
    pub fn clr_match_flag(&self) -> &Self {
        match self.index {
            0 => { self.periph.set_icr(|r| r.set_tamcint(1)); },
            1 => { self.periph.set_icr(|r| r.set_tbmcint(1)); },
            _ => panic!("Invalid channel index"),
        };
        self
    }   
    pub fn dma_done_flag(&self) -> bool {
        self.periph.ris().dmaris(self.index) != 0
    }
    pub fn clr_dma_done_flag(&self) -> &Self {
        self.periph.set_icr(|r| r.set_dmaint(self.index, 1));
        self
    }       
}


impl Delay<u32> for TimerPeriph {    
    fn delay(&self, value: u32) -> &Self {
        self
            .clr_timeout()
            .start_once(value)
            .wait_timeout()
    }
}

impl Stop for TimerPeriph {
    fn stop(&self) -> &Self {
        self.with_ctl(|r| r.set_ten(0, 0))
    }
}

impl Running for TimerPeriph {
    fn running(&self) -> bool {
        self.ctl().ten(0) != 0
    }
}

impl Period<u32> for TimerPeriph {
    fn period(&self) -> u32 {
        self.tilr(0).tilr().value()
    }
}

impl SetPeriod<u32> for TimerPeriph {
    fn set_period(&self, value: u32) -> &Self {
        self.set_tilr(0, |r| r.set_tilr(value))
    }
}

impl Counter<u32> for TimerPeriph {
    fn counter(&self) -> u32 {
        self.tv(0).tv().value()
    }
}

impl Timeout for TimerPeriph {
    fn test_timeout(&self) -> bool {
        self.ris().ttoris(0) != 0
    }

    fn clr_timeout(&self) -> &Self {
        self.set_icr(|r| r.set_ttocint(0, 1))
    }
}


impl Start<u32> for TimerPeriph {
    fn start(&self, value: u32) -> &Self {
        self.start_down(value)
    }
}

impl StartOnce<u32> for TimerPeriph {
    fn start_once(&self, value: u32) -> &Self {
        self.start_down_once(value)
    }
}

impl StartDown<u32> for TimerPeriph {
    fn start_down(&self, value: u32) -> &Self {    
        // disable timer a        
        self.with_ctl(|r| r.set_ten(0, 0));
        // set 32 bit mode
        self.set_cfg(|r| r);
        // set timer a mode = repeat
        self.set_tmr(0, |r| r.set_tcdir(0).set_tmr(0x02));
        // set timer a load register
        self.set_tilr(0, |r| r.set_tilr(value));
        // enable timer a
        self.with_ctl(|r| r.set_ten(0, 1));
        self
    }
}

impl StartUp<u32> for TimerPeriph {
    fn start_up(&self, value: u32) -> &Self {
        // disable timer a        
        self.with_ctl(|r| r.set_ten(0, 0));
        // set 32 bit mode
        self.set_cfg(|r| r);
        // set timer a mode = repeat
        self.set_tmr(0, |r| r.set_tcdir(1).set_tmr(0x02));
        // set timer a load register
        self.set_tilr(0, |r| r.set_tilr(value));
        // enable timer a
        self.with_ctl(|r| r.set_ten(0, 1));        
        self
    }
}

impl StartDownOnce<u32> for TimerPeriph {
    fn start_down_once(&self, value: u32) -> &Self {
        // disable timer a        
        self.with_ctl(|r| r.set_ten(0, 0));
        // set 32 bit mode
        self.set_cfg(|r| r);
        // set timer a mode = one-shot
        self.set_tmr(0, |r| r.set_tcdir(0).set_tmr(0x01));        
        // set timer a load register
        self.set_tilr(0, |r| r.set_tilr(value));
        // enable timer a
        self.with_ctl(|r| r.set_ten(0, 1));               
        self
    }
}

impl StartUpOnce<u32> for TimerPeriph {
    fn start_up_once(&self, value: u32) -> &Self {
        // disable timer a        
        self.with_ctl(|r| r.set_ten(0, 0));
        // set 32 bit mode
        self.set_cfg(|r| r);
        // set timer a mode = one-shot
        self.set_tmr(0, |r| r.set_tcdir(1).set_tmr(0x01));        
        // set timer a load register
        self.set_tilr(0, |r| r.set_tilr(value));
        // enable timer a
        self.with_ctl(|r| r.set_ten(0, 1));         
        self
    }
}

impl Prescale<u8> for TimerPeriph {
    fn prescale(&self) -> u8 {
        self.tpr(0).tpsr().value()
    }
}
impl SetPrescale<u8> for TimerPeriph {
    fn set_prescale(&self, value: u8) -> &Self {
        self.set_tpr(0, |r| r.set_tpsr(value))
    }
}

impl SetCounter<u32> for TimerPeriph {
    fn set_counter(&self, value: u32) -> &Self {
        self.set_tv(0, |r| r.set_tv(value))
    }
}

impl Compare<u32> for TimerPeriph {
    fn compare(&self) -> u32 {
        self.tmtchr(0).tmtchr().value()
    }

    fn set_compare(&self, value: u32) -> &Self {
        self.set_tmtchr(0, |r| r.set_tmtchr(value))
    }

    fn test_compare(&self) -> bool {
        self.ris().tamris() != 0
    }

    fn clr_compare(&self) -> &Self {
        self.set_icr(|r| r.set_tamcint(1))
    }

}