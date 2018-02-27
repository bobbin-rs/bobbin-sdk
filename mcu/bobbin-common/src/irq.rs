pub trait IrqType {}

pub trait IrqNumber {
    fn number(&self) -> u8;
}

pub trait Irq<IT: IrqType, IN: IrqNumber> {
    fn irq(&self) -> IN;
}
