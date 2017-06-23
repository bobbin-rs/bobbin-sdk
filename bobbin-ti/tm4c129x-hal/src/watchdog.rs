pub use chip::watchdog::*;
pub use sysctl::{SysctlEnabled, SysctlReady};

pub trait WatchdogExt {
}

impl<T> WatchdogExt for Periph<T> {    
}
