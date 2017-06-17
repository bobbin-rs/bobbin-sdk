pub use chip::timer::*;
pub use sysctl::SysctlEnabled;

pub trait TimerExt {
    fn delay(&self, value: u32) -> &Self;
}

impl<T> TimerExt for Periph<T> {    
    fn delay(&self, value: u32) -> &Self {
        // disable timer a        
        self.with_ctl(|r| r.set_ten(0, 0));
        // set 32 bit mode
        self.set_cfg(Cfg(0x0));
        // set timer a mode = one-shot
        self.set_tmr(0, Tmr(0).set_tmr(0x01));        
        // set timer a load register
        self.set_tilr(0, Tilr(0).set_tilr(value));
        // clear timeout interrupt
        self.set_icr(Icr(0).set_ttocint(0, 1));
        // enable timer a
        self.with_ctl(|r| r.set_ten(0, 1));        
        // wait for timer a timeout
        while self.ris().ttoris(0) == 0 {}
        // clear timeout interrupt
        self.set_icr(Icr(0).set_ttocint(0, 1));
        self
    }
}

pub trait TimerChExt {    
}

impl<P, T> TimerChExt for Channel<P, T> {
    
}