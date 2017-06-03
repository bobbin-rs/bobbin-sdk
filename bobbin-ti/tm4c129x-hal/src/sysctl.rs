pub use chip::sysctl::*;

pub trait SysctlEnabled {
    fn sysctl_enabled(&self) -> bool;
    fn sysctl_set_enabled(&self, value: bool) -> &Self;
    fn sysctl_enable(&self) -> &Self { self.sysctl_set_enabled(true); self }
    fn sysctl_disable(&self) -> &Self { self.sysctl_set_enabled(false); self }
}

impl<P> SysctlEnabled for P where P: Rcgc {
    fn sysctl_enabled(&self) -> bool {
        self.rcgc() != 0
    }
    fn sysctl_set_enabled(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.set_rcgc(value);
        self
    }
}