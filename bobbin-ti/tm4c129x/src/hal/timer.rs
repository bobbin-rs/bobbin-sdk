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
        self.0.tmr(self.1)
    }
    pub fn set_tmr(&self, value: Tmr) -> &Self {
        self.0.set_tmr(self.1, |_| value);
        self
    }
    pub fn with_tmr<F: FnOnce(Tmr) -> Tmr>(&self, f: F) -> &Self {
        self.0.with_tmr(self.1, f);
        self
    }
    pub fn enabled(&self) -> bool {
        self.0.ctl().ten(self.1) != 0
    }
    pub fn set_enabled(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.0.with_ctl(|r| r.set_ten(self.1, value));
        self
    }
    pub fn reload(&self) -> u32 {
        self.0.tilr(self.1).tilr().value()
    }
    pub fn set_reload(&self, value: u32) -> &Self {
        self.0.set_tilr(self.1, |_| Tilr(value));
        self
    }

    pub fn prescale(&self) -> u8 {
        self.0.tpr(self.1).tpsr().value()
    }

    pub fn set_prescale(&self, value: u8) -> &Self {
        self.0.set_tpr(self.1, |r| r.set_tpsr(value));
        self
    }

    pub fn compare(&self) -> u32 {
        self.0.tmtchr(self.1).tmtchr().value()
    }
    pub fn set_compare(&self, value: u32) -> &Self {
        self.0.set_tmtchr(self.1, |_| Tmtchr(value));
        self
    }    
    pub fn value(&self) -> u32 {
        self.0.tv(self.1).tv().value()
    }
    pub fn set_value(&self, value: u32) -> &Self {
        self.0.set_tv(self.1, |_| Tv(value));
        self
    }        
    pub fn match_dmaen(&self) -> bool {
        self.0.dmaev().tmdmaen(self.1) != 0
    }
    pub fn set_match_dmaen(&self, value: bool) -> &Self {
        self.0.with_dmaev(|r| r.set_tmdmaen(self.1, value));
        self
    }
    #[inline]
    pub fn timeout_flag(&self) -> bool {
        self.0.ris().ttoris(self.1) != 0
    }
    #[inline]
    pub fn clr_timeout_flag(&self) -> &Self {
        self.0.set_icr(|r| r.set_ttocint(self.1, 1));
        self
    }
    #[inline]
    pub fn match_flag(&self) -> bool {
        match self.1 {
            0 => self.0.ris().tamris() != 0,
            1 => self.0.ris().tbmris() != 0,            
            _ => panic!("Invalid channel index"),
        }
    }
    #[inline]
    pub fn clr_match_flag(&self) -> &Self {
        match self.1 {
            0 => { self.0.set_icr(|r| r.set_tamcint(1)); },
            1 => { self.0.set_icr(|r| r.set_tbmcint(1)); },
            _ => panic!("Invalid channel index"),
        };
        self
    }   
    pub fn dma_done_flag(&self) -> bool {
        self.0.ris().dmaris(self.1) != 0
    }
    pub fn clr_dma_done_flag(&self) -> &Self {
        self.0.set_icr(|r| r.set_dmaint(self.1, 1));
        self
    }       
}


impl Delay<u32> for TimerPeriph {    
    fn delay(&self, value: u32) -> &Self {
        self
            .start_down_once(value)
            .clr_timeout_flag()
            .wait_timeout_flag()
    }
}

impl Timer<u32> for TimerPeriph {
    fn stop(&self) -> &Self {
        self.with_ctl(|r| r.set_ten(0, 0))
    }
    fn running(&self) -> bool {
        self.ctl().ten(0) != 0
    }

    fn period(&self) -> u32 {
        self.tilr(0).tilr().value()
    }
    fn set_period(&self, value: u32) -> &Self {
        self.set_tilr(0, |r| r.set_tilr(value))
    }

    fn counter(&self) -> u32 {
        self.tv(0).tv().value()
    }

    fn timeout_flag(&self) -> bool {
        self.ris().ttoris(0) != 0
    }
    fn clr_timeout_flag(&self) -> &Self {
        self.set_icr(|r| r.set_ttocint(0, 1))
    }
}


impl Start<u32> for TimerPeriph {
    fn start(&self, value: u32) -> &Self {
        self.start_down(value)
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

    fn compare_flag(&self) -> bool {
        self.ris().tamris() != 0
    }

    fn clr_compare_flag(&self) -> &Self {
        self.set_icr(|r| r.set_tamcint(1))
    }

}