use ::chip::fpu::*;

pub fn init() {
    unsafe {
        FPU.set_cpacr(Cpacr(0).set_cp10(0b11).set_cp11(0b11));
    }
}
