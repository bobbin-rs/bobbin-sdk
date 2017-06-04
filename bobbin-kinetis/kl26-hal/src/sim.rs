pub use ::chip::sim::*;

pub trait SimEnabled {
    fn sim_enabled(&self) -> bool;
    fn sim_set_enabled(&self, value: bool) -> &Self;
    fn sim_enable(&self) -> &Self { self.sim_set_enabled(true); self }
    fn sim_disable(&self) -> &Self { self.sim_set_enabled(false); self }
}

impl<P> SimEnabled for P where P: En {
    fn sim_enabled(&self) -> bool {
        self.en() != 0
    }
    fn sim_set_enabled(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.set_en(value);
        self
    }
}

pub trait SimSrc {
    fn sim_src(&self) -> u32;
    fn sim_set_src(&self, value: u32) -> &Self;
}

impl<P> SimSrc for P where P: Src {
    fn sim_src(&self) -> u32 {
        self.src()
    }
    fn sim_set_src(&self, value: u32) -> &Self {
        self.set_src(value);
        self
    }
}