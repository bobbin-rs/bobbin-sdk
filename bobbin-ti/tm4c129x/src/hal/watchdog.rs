pub use bobbin_common::enabled::*;
pub use bobbin_common::watchdog::*;
pub use chip::watchdog::*;
pub use super::sysctl::{SysctlEnabled, SysctlReady};

impl Unlock for WatchdogPeriph {
    fn unlock(&self) -> &Self {
        self.set_lock(|r| r.set_lock(0x1ACC_E551))
    }
}

impl ::bobbin_common::watchdog::Lock for WatchdogPeriph {
    fn lock(&self) -> &Self {
        self.set_lock(|r| r.set_lock(0x00000000))
    }
}

impl Timeout<u32> for WatchdogPeriph {
    fn timeout(&self) -> u32 {
        self.load().load().value()
    }
}

impl SetTimeout<u32> for WatchdogPeriph {
    fn set_timeout(&self, value: u32) -> &Self {
        self.with_load(|r| r.set_load(value))
    }
}

impl Counter<u32> for WatchdogPeriph {
    fn counter(&self) -> u32 {
        self.value().value().value()
    }
}


impl Refresh for WatchdogPeriph {
    fn refresh(&self) -> &Self {
        self.set_load(|_| self.load())
    }
}