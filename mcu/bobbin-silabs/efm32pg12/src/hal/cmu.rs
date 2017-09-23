pub use ::chip::cmu::*;


pub trait CmuEnabled {
    fn cmu_enabled(&self) -> bool;
    fn cmu_set_enabled(&self, value: bool) -> &Self;
    fn cmu_enable(&self) -> &Self { self.cmu_set_enabled(true); self }
    fn cmu_disable(&self) -> &Self { self.cmu_set_enabled(false); self }
}

impl<P> CmuEnabled for P where P: Clken {
    fn cmu_enabled(&self) -> bool {
        self.clken() != 0
    }
    fn cmu_set_enabled(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.set_clken(value);
        self
    }
}
