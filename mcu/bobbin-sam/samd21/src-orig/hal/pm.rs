pub use chip::pm::*;

pub trait PmEnabled {
    fn pm_enabled(&self) -> bool;
    fn pm_set_enabled(&self, value: bool) -> &Self;
    fn pm_enable(&self) -> &Self { self.pm_set_enabled(true); self }
    fn pm_disable(&self) -> &Self { self.pm_set_enabled(false); self }
}

impl<P> PmEnabled for P where P: En {
    fn pm_enabled(&self) -> bool {
        self.en() != 0
    }
    fn pm_set_enabled(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.set_en(value);
        self
    }
}