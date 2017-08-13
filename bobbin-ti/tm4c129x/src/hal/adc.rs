pub use ::chip::adc::*;
pub use super::sysctl::SysctlEnabled;
pub use super::gpio::ModeAin;

pub trait AdcExt {
    fn init(&self) -> &Self;
}

impl<T> AdcExt for Periph<T> {
    fn init(&self) -> &Self {
        self
    }
}