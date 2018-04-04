pub enum IrqResult {
    End,
    Continue,
}

pub trait HandleIrq {
    unsafe fn handle_irq(&mut self, irq: u8) -> IrqResult;
}


pub enum RegisterError {
    Unavailable,
}

pub trait RegisterExc {
    type Handle;
    fn register_exc(&self, irq: u8, handler: *mut HandleIrq) -> Result<Self::Handle, RegisterError>;
}

pub trait RegisterIrq {
    type Handle;
    fn register_irq(&self, irq: u8, handler: *mut HandleIrq) -> Result<Self::Handle, RegisterError>;
}

pub trait EnableIrq {
    fn enable_irq(&self, irq: u8);
}
pub trait DisableIrq {
    fn disable_irq(&self, irq: u8);
}