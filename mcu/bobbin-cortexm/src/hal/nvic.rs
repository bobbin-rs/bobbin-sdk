//! Extends the ```chip::nvic``` module.

pub use bobbin_common::{Handler, Irq, Poll};
pub use ::chip::nvic::*;

/// Returns `true` if IRQ `irq` is enabled.
pub fn enabled(irq: usize) -> bool {
    NVIC.iser(irq >> 5).setena(irq & 0b11111) != 0
}

/// `true` enables IRQ `irq`, `false` disables the IRQ.
pub fn set_enabled(irq: usize, value: bool) {
    if value {
        NVIC.set_iser(irq >> 5, |r| r.set_setena(irq & 0b11111, 1));
    } else {
        NVIC.set_icer(irq >> 5, |r| r.set_clrena(irq & 0b11111, 1));
    }        
}

/// Returns `true` if IRQ `irq` is pending.
pub fn pending(irq: usize) -> bool {
    NVIC.ispr(irq >> 5).setpend(irq & 0b11111) != 0
}

/// `true` sets IRQ `irq` pending, `false` clears the IRQ.
pub fn set_pending(irq: usize, value: bool) {
    if value {
        NVIC.set_ispr(irq >> 5, |r| r.set_setpend(irq & 0b11111, 1));
    } else {
        NVIC.set_icpr(irq >> 5, |r| r.set_clrpend(irq & 0b11111, 1));
    }        
}

/// Returns `true` if IRQ `irq` is active.
pub fn active(irq: usize) -> bool {
    NVIC.iabr(irq >> 5).active(irq & 011111) != 0
}

/// Returns the priority level of IRQ `irq`.
pub fn priority(irq: usize) -> u8 {
    NVIC.ipr(irq >> 4).pri(irq & 0b1111).value()
}

/// Set the priority level of IRQ `irq`.
pub fn set_priority(irq: usize, value: u8) {
    NVIC.with_ipr(irq >> 4, |r| r.set_pri(irq & 0b1111, value));
}

/// Trigger IRQ `irq`.
pub fn trigger_interrupt(irq: usize) {
    NVIC.set_stir(|r| r.set_intid(irq));
}


pub trait NvicEnabled {
    fn nvic_enabled(&self) -> bool;
    fn nvic_set_enabled(&self, value: bool) -> &Self;
    fn nvic_enable(&self) -> &Self {
        self.nvic_set_enabled(true);
        self
    }
    fn nvic_disable(&self) -> &Self {
        self.nvic_set_enabled(false);
        self
    }
}

impl<I: Irq> NvicEnabled for I {
    fn nvic_enabled(&self) -> bool {
        enabled(self.irq_num() as usize)
    }
    fn nvic_set_enabled(&self, value: bool) -> &Self {
        set_enabled(self.irq_num() as usize, value);
        self
    }
}
