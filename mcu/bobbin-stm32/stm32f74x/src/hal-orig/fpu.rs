use ::chip::fpu::*;

pub fn init() {
    FPU.set_cpacr(|_| Cpacr(0).set_cp10(0b11).set_cp11(0b11));
}
