//! Extends the chip::scb module.

pub use bobbin_common::Handler;
pub use chip::scb::*;

// VTOR

/// Returns bits [29:7] of the offset of the vector table base.
pub fn tbloff() -> u32 {
    SCB.vtor().tbloff().into()
}

/// Sets bits [29:7] of the offset of the vector table base.
pub fn set_tbloff(value: u32) {
    SCB.set_vtor(|r| r.set_tbloff(value));
}

impl Scb {
    pub fn set_irq_handler(&self, irq_num: usize, f: Option<Handler>) {
        assert!(self.irq_handler(irq_num).is_some() != f.is_some(),"IRQ Handler {} is already set.", irq_num);
        let addr = (self.vtor().0 as usize) + ((irq_num + 16) * 4);
        unsafe {            
            ::core::ptr::write_volatile(addr as *mut Option<Handler>, f)
        }
    }

    pub fn irq_handler(&self, irq_num: usize) -> Option<Handler> {
        let addr = (self.vtor().0 as usize) + ((irq_num + 16) * 4);
        unsafe { 
            ::core::ptr::read_volatile(addr as *const Option<Handler>)
        }
    }
}