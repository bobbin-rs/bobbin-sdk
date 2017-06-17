pub use chip::pwm::*;
pub use sysctl::SysctlEnabled;

pub trait PwmExt {
}

impl<T> PwmExt for Periph<T> {    
}

pub trait PwmChExt {    
}

impl<P, T> PwmChExt for Channel<P, T> {
}