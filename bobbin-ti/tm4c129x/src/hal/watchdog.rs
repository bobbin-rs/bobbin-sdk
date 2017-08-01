pub use chip::watchdog::*;
pub use super::sysctl::{SysctlEnabled, SysctlReady};

pub trait WatchdogExt {
}

impl<T> WatchdogExt for Periph<T> {    
}
