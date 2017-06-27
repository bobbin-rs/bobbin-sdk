pub use chip::tcc::*;
pub use pm::PmEnabled;

pub trait TccExt {
}

impl<T> TccExt for Periph<T> {
}

pub trait TccChExt {
    fn cc(&self) -> u32;
    fn set_cc(&self, u32) -> &Self;
}

impl<P, T> TccChExt for Channel<P, T> {
    fn cc(&self) -> u32 {
        self.periph.cc(self.index).cc()
    }
    fn set_cc(&self, value: u32) -> &Self {
        self.periph.with_cc(self.index, |r| r.set_cc(value));
        self
    }
}