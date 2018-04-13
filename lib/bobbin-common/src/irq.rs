pub trait IrqNumber : Default {
    fn irq_number() -> u8;
}

pub trait IrqType : Default {}

pub trait Irq<IT: IrqType> : Default {
    type Output: IrqNumber;
    fn irq_number_for(&self, IT) -> u8 { Self::Output::irq_number() }
}
