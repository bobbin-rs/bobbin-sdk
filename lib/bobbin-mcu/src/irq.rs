//! Abstracts Interrupt assignments for peripherals and channels.
//! 
//! Peripherals and Peripheral Channels are often assigned to one or more
//! interrupts, and there are often multiple peripherals assigned to the
//! same interrupt.
//! 
//! `IrqNumber` is implemented by types representing an interrupt number; there
//! should generally be one type implementing IrqNumber for each valid interrupt
//! on a processor.
//! 
//! `IrqType` allows distinguishing between interrupts for peripherals and channels
//! that support more than one interrupt. For instance, a peripheral type might have
//! dedicated Transmit and Receive interrupts, each of which would be associated with a
//! distinct interrupt.
//! 
//! `Irq<IT: IrqType>` associates a specific interrupt number with a specific interrupt
//! type for a specific peripheral. 

/// Marker Trait for an interrupt number.
pub trait IrqNumber : Default {
    fn irq_number() -> u8;
}

/// Marker trait for an interrupt type.
pub trait IrqType : Default {}

/// Associates a specific interrupt number with a specific interrupt type, implemented
/// for a Peripheral or Channel.
pub trait Irq<IT: IrqType> : Default {
    type Output: IrqNumber;
    /// Returns the interrupt number associated with the IRQ Type and peripheral.
    fn irq_number_for(&self, IT) -> u8 { Self::Output::irq_number() }
}
