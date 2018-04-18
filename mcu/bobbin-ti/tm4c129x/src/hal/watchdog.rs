pub use bobbin_common::enabled::*;
pub use bobbin_hal::watchdog::*;
pub use chip::watchdog::*;
pub use super::sysctl::{SysctlEnabled, SysctlReady};

impl Unlock for WatchdogPeriph {
    fn unlock(&self) -> &Self {
        self.set_lock(|r| r.set_lock(0x1ACC_E551))
    }
}

impl ::bobbin_hal::watchdog::Lock for WatchdogPeriph {
    fn lock(&self) -> &Self {
        self.set_lock(|r| r.set_lock(0x00000000))
    }
}

impl Enable for WatchdogPeriph {
    fn enable(&self) -> &Self {
        self.with_ctl(|r| r.set_resen(1))
    }
}

impl Disable for WatchdogPeriph {
    fn disable(&self) -> &Self {
        self.with_ctl(|r| r.set_resen(0))
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