pub use bobbin_common::timer::*;
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

pub trait TimerChExt {
    fn tmr(&self) -> Tmr;
    fn set_tmr(&self, Tmr) -> &Self;
    fn with_tmr<F: FnOnce(Tmr) -> Tmr>(&self, F) -> &Self;
    fn enabled(&self) -> bool;
    fn set_enabled(&self, value: bool) -> &Self;
    fn reload(&self) -> u32;
    fn set_reload(&self, value: u32) -> &Self;
    fn compare(&self) -> u32;
    fn set_compare(&self, value: u32) -> &Self;
    fn prescale(&self) -> u8;
    fn set_prescale(&self, value: u8) -> &Self;
    fn value(&self) -> u32;
    fn set_value(&self, value: u32) -> &Self;
    fn match_dmaen(&self) -> bool;
    fn set_match_dmaen(&self, value: bool) -> &Self;
    fn timeout_flag(&self) -> bool;
    fn clr_timeout_flag(&self) -> &Self;
    fn match_flag(&self) -> bool;
    fn clr_match_flag(&self) -> &Self;
    fn dma_done_flag(&self) -> bool;
    fn clr_dma_done_flag(&self) -> &Self;
}

impl<P, T> TimerChExt for Channel<P, T> {
    fn tmr(&self) -> Tmr {
        self.periph.tmr(self.index)
    }
    fn set_tmr(&self, value: Tmr) -> &Self {
        self.periph.set_tmr(self.index, |_| value);
        self
    }
    fn with_tmr<F: FnOnce(Tmr) -> Tmr>(&self, f: F) -> &Self {
        self.periph.with_tmr(self.index, f);
        self
    }
    fn enabled(&self) -> bool {
        self.periph.ctl().ten(self.index) != 0
    }
    fn set_enabled(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.periph.with_ctl(|r| r.set_ten(self.index, value));
        self
    }
    fn reload(&self) -> u32 {
        self.periph.tilr(self.index).tilr().value()
    }
    fn set_reload(&self, value: u32) -> &Self {
        self.periph.set_tilr(self.index, |_| Tilr(value));
        self
    }

    fn prescale(&self) -> u8 {
        self.periph.tpr(self.index).tpsr().value()
    }

    fn set_prescale(&self, value: u8) -> &Self {
        self.periph.set_tpr(self.index, |r| r.set_tpsr(value));
        self
    }

    fn compare(&self) -> u32 {
        self.periph.tmtchr(self.index).tmtchr().value()
    }
    fn set_compare(&self, value: u32) -> &Self {
        self.periph.set_tmtchr(self.index, |_| Tmtchr(value));
        self
    }    
    fn value(&self) -> u32 {
        self.periph.tv(self.index).tv().value()
    }
    fn set_value(&self, value: u32) -> &Self {
        self.periph.set_tv(self.index, |_| Tv(value));
        self
    }        
    fn match_dmaen(&self) -> bool {
        self.periph.dmaev().tmdmaen(self.index) != 0
    }
    fn set_match_dmaen(&self, value: bool) -> &Self {
        self.periph.with_dmaev(|r| r.set_tmdmaen(self.index, value));
        self
    }
    #[inline]
    fn timeout_flag(&self) -> bool {
        self.periph.ris().ttoris(self.index) != 0
    }
    #[inline]
    fn clr_timeout_flag(&self) -> &Self {
        self.periph.set_icr(|r| r.set_ttocint(self.index, 1));
        self
    }
    #[inline]
    fn match_flag(&self) -> bool {
        match self.index {
            0 => self.periph.ris().tamris() != 0,
            1 => self.periph.ris().tbmris() != 0,            
            _ => panic!("Invalid channel index"),
        }
    }
    #[inline]
    fn clr_match_flag(&self) -> &Self {
        match self.index {
            0 => self.periph.set_icr(|r| r.set_tamcint(1)),
            1 => self.periph.set_icr(|r| r.set_tbmcint(1)),
            _ => panic!("Invalid channel index"),
        };
        self
    }   
    fn dma_done_flag(&self) -> bool {
        self.periph.ris().dmaris(self.index) != 0
    }
    fn clr_dma_done_flag(&self) -> &Self {
        self.periph.set_icr(|r| r.set_dmaint(self.index, 1));
        self
    }       
}


impl<T> Delay<u32> for Periph<T> {    
    fn delay(&self, value: u32) -> &Self {
        self
            .start_down_once(value)
            .clr_timeout_flag()
            .wait_timeout_flag()
    }
}

impl<T> Timer<u32> for Periph<T> {
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


impl<T> Start<u32> for Periph<T> {
    fn start(&self, value: u32) -> &Self {
        self.start_down(value)
    }
}

impl<T> StartDown<u32> for Periph<T> {
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

impl<T> StartUp<u32> for Periph<T> {
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

impl<T> StartDownOnce<u32> for Periph<T> {
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

impl<T> StartUpOnce<u32> for Periph<T> {
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

impl<T> Prescale<u8> for Periph<T> {
    fn prescale(&self) -> u8 {
        self.tpr(0).tpsr().value()
    }
    fn set_prescale(&self, value: u8) -> &Self {
        self.set_tpr(0, |r| r.set_tpsr(value))
    }
}

impl<T> SetCounter<u32> for Periph<T> {
    fn set_counter(&self, value: u32) -> &Self {
        self.set_tv(0, |r| r.set_tv(value))
    }
}

impl<T> Compare<u32> for Periph<T> {
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