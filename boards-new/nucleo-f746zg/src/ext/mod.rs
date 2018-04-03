use common::dispatch::*;

impl RegisterExc for ::NucleoF746zg {
    fn register_exc(&self, exc: u8, handler: *mut HandleIrq) -> Result<IrqHandle, RegisterError> {
        Dispatcher::register_handler(IrqHandler::new(exc, handler)).ok_or(RegisterError::Unavailable)
    }
}

impl RegisterIrq for ::NucleoF746zg {
    fn register_irq(&self, irq: u8, handler: *mut HandleIrq) -> Result<IrqHandle, RegisterError> {
        Dispatcher::register_handler(IrqHandler::new(irq + 16, handler)).ok_or(RegisterError::Unavailable)
    }
}