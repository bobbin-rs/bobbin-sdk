pub use chip::timer::*;
pub use sysctl::SysctlEnabled;

pub trait TimerExt {
    fn delay(&self, value: u32) -> &Self;
}

impl<T> TimerExt for Periph<T> {    
    fn delay(&self, value: u32) -> &Self {
        // disable timer a        
        self.with_ctl(|r| r.set_taen(0));
        // set 32 bit mode
        self.set_cfg(Cfg(0x0));
        // set timer a mode = one-shot
        self.set_tamr(Tamr(0).set_tamr(0x01));        
        // set timer a load register
        self.set_tailr(Tailr(0).set_tailr(value));
        // clear timeout interrupt
        self.set_icr(Icr(0).set_tatocint(1));
        // enable timer a
        self.with_ctl(|r| r.set_taen(1));        
        // wait for timer a timeout
        while self.ris().tatoris() == 0 {}
        // clear timeout interrupt
        self.set_icr(Icr(0).set_tatocint(1));
        self
    }
}