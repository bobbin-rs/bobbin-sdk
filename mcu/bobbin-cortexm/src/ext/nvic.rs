use bobbin_sys::irq_dispatch::EnableDisableIrq;
use nvic::NvicPeriph;

impl NvicPeriph {
    /// Returns `true` if IRQ `irq` is enabled.
    pub fn enabled(&self, irq: u8) -> bool {
        self.iser(irq >> 5).setena(irq & 0b11111) != 0
    }

    /// `true` enables IRQ `irq`, `false` disables the IRQ.
    pub fn set_enabled(&self, irq: u8, value: bool) {
        if value {
            self.set_iser(irq >> 5, |r| r.set_setena(irq & 0b11111, 1));
        } else {
            self.set_icer(irq >> 5, |r| r.set_clrena(irq & 0b11111, 1));
        }        
    }

    /// Returns `true` if IRQ `irq` is pending.
    pub fn pending(&self, irq: u8) -> bool {
        self.ispr(irq >> 5).setpend(irq & 0b11111) != 0
    }

    /// `true` sets IRQ `irq` pending, `false` clears the IRQ.
    pub fn set_pending(&self, irq: u8, value: bool) {
        if value {
            self.set_ispr(irq >> 5, |r| r.set_setpend(irq & 0b11111, 1));
        } else {
            self.set_icpr(irq >> 5, |r| r.set_clrpend(irq & 0b11111, 1));
        }        
    }

    /// Returns `true` if IRQ `irq` is active.
    pub fn active(&self, irq: u8) -> bool {
        self.iabr(irq >> 5).active(irq & 0b11111) != 0
    }

    /// Returns the priority level of IRQ `irq`.
    pub fn priority(&self, irq: u8) -> u8 {
        self.ipr(irq >> 4).pri(irq & 0b1111).value()
    }

    /// Set the priority level of IRQ `irq`.
    pub fn set_priority(&self, irq: u8, value: u8) {
        self.with_ipr(irq >> 4, |r| r.set_pri(irq & 0b1111, value));
    }

    /// Trigger IRQ `irq`.
    pub fn trigger_interrupt(&self, irq: u8) {
        self.set_stir(|r| r.set_intid(irq));
    }
}

impl EnableDisableIrq for NvicPeriph {
    fn enable_irq(&self, irq: u8) {
        self.set_enabled(irq - 16, true);
    }
    fn disable_irq(&self, irq: u8) {
        self.set_enabled(irq - 16, false);
    }

}