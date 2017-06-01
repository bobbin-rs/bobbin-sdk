pub use ::chip::sim::*;

pub trait SimExt {
    fn enabled<P: En>(&self, p: &P) -> bool;
    fn set_enabled<P: En>(&self, p: &P, value: bool) -> &Self;
}

impl SimExt for Sim {
    fn enabled<P: En>(&self, p: &P) -> bool {
        self.en(p) != 0
    }
    fn set_enabled<P: En>(&self, p: &P, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.set_en(p, value);
        self
    }
    
}

pub fn set_enabled<P: En>(p: &P, value: bool) {
    SIM.set_enabled(p, value);
}

pub fn enable<P: En>(p: &P) {
    SIM.set_enabled(p, true);
}

pub fn disable<P: En>(p: &P) {
    SIM.set_enabled(p, false);
}

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