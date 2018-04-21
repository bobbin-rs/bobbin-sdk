//! Simple serial input and output traits.
//!
//! TODO: Add Error and Results to traits

/// Operations on a device that can write `T`.
pub trait SerialTx<T: Copy> {
    /// Returns true if the device can currently accept a word to transmit.
    fn can_tx(&self) -> bool;

    /// Transmit a word. Behavior is undefined if `can_tx()` is not true.
    fn tx(&self, c: T) -> &Self;

    /// Busy wait until `can_tx()` is true.
    fn wait_tx(&self) -> &Self {
        while !self.can_tx() {}
        self
    }

    /// Busy wait until `can_tx()` is true, then transmit a word.
    fn putc(&self, c: T) -> &Self {
        self.wait_tx().tx(c)
    }

    /// Attempt to transmit the word `c`, returning true if successful.
    fn try_putc(&self, c: T) -> bool {
        if self.can_tx() {
            self.tx(c);
            true
        } else {
            false
        }
    }    

    /// Blocking write of the slice `buf`.
    fn write(&self, buf: &[T]) -> &Self {
        for b in buf.iter() {
            self.putc(*b);
        }
        self
    }    
}

/// Operations on a device that can read `T`.
pub trait SerialRx<T: Copy> {
    /// Returns true if there is a word available to read.
    fn can_rx(&self) -> bool;

    /// Returns a word. Behavior is undefined if `can_rx()` is not true.
    fn rx(&self) -> T;

    /// Busy wait until `can_rx()` is true.
    fn wait_rx(&self) -> &Self {
        while !self.can_rx() {}
        self
    }

    /// Busy wait until `can_rx()` is true, then return the word that was read.
    fn getc(&self) -> T {
        self.wait_rx().rx()
    }

    /// Attempt to read a word, returning `None` if `can_rx()` was false.
    fn try_getc(&self) -> Option<T> {
        if self.can_rx() {
            Some(self.rx())
        } else {
            None
        }
    }        
}

/// Traits for managing transmit interrupts. For SerialTxIrq, the interrupt is
/// the one that indicates that a it is possible to transmit a word. This interrupt
/// should fire whenever `can_tx()` transitions from false to true.
pub trait SerialTxIrq {
    /// Returns true if the tx interrupt is enabled.
    fn tx_irq(&self) -> bool;
    /// Sets the tx interrupt state to `value`.
    fn set_tx_irq(&self, value: bool) -> &Self;
    /// Enables the tx interrupt.
    fn enable_tx_irq(&self) -> &Self { self.set_tx_irq(true) }
    /// Disables the tx interrupt.
    fn disable_tx_irq(&self) -> &Self { self.set_tx_irq(false) }
}

/// Traits for managing receive interrupts. For SerialRxIrq, the interrupt is
/// the one that indicates that a it is possible to receive a word. This interrupt
/// should fire whenever `can_rx()` transitions from false to true.
pub trait SerialRxIrq {
    /// Returns true if the rx interrupt is enabled.
    fn rx_irq(&self) -> bool;
    /// Sets the rx interrupt state to `value`.
    fn set_rx_irq(&self, value: bool) -> &Self;
    /// Enables the rx interrupt.
    fn enable_rx_irq(&self) -> &Self { self.set_rx_irq(true) }
    /// Disables the rx interrupt.
    fn disable_rx_irq(&self) -> &Self { self.set_rx_irq(false) }
}
