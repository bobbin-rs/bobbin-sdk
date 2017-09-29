pub use ::chip::fpu::*;

pub fn init() {
    FPU.set_cpacr(|r| r.set_cp10(0b11).set_cp11(0b11));
}
