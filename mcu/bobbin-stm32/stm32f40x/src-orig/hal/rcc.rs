pub use ::chip::rcc::*;

pub trait RccExt {
    fn enabled<P: En>(&self, p: &P) -> bool;
    fn set_enabled<P: En>(&self, p: &P, value: bool) -> &Self;
}

impl RccExt for Rcc {
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
    RCC.set_enabled(p, value);
}

pub fn enable<P: En>(p: &P) {
    RCC.set_enabled(p, true);
}

pub fn disable<P: En>(p: &P) {
    RCC.set_enabled(p, false);
}

pub trait RccEnabled {
    fn rcc_enabled(&self) -> bool;
    fn rcc_set_enabled(&self, value: bool) -> &Self;
    fn rcc_enable(&self) -> &Self { self.rcc_set_enabled(true); self }
    fn rcc_disable(&self) -> &Self { self.rcc_set_enabled(false); self }
}

impl<P> RccEnabled for P where P: En {
    fn rcc_enabled(&self) -> bool {
        self.en() != 0
    }
    fn rcc_set_enabled(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.set_en(value);
        self
    }
}
