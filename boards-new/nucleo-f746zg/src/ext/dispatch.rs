pub enum IrqResult {
    End,
    Continue,
}

pub trait HandleIrq {
    unsafe fn handle_irq(&mut self, irq: u8) -> IrqResult;
}

#[derive(Debug)]
pub enum RegisterError {
    Unavailable,
}

pub trait RegisterExc<H: HandleIrq> {
    type Handle;
    fn register_exc(&self, irq: u8, handler: H) -> Result<Self::Handle, RegisterError>;
}

pub trait RegisterIrq<H: HandleIrq> {
    type Handle;
    fn register_irq(&self, irq: u8, handler: H) -> Result<Self::Handle, RegisterError>;
}

pub trait EnableIrq {
    fn enable_irq(&self, irq: u8);
}
pub trait DisableIrq {
    fn disable_irq(&self, irq: u8);
}