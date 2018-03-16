pub use bobbin_common::Channel;
pub use chip::dmac::*;
use super::pm::PM;

impl Dmac {
    pub fn pm_set_enabled(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        PM.with_ahbmask(|r| r.set_dmac(value));
        PM.with_apbbmask(|r| r.set_dmac(value));
        self
    }    
}