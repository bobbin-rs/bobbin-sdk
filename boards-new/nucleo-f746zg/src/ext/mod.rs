use common::dispatch::*;
use mcu::nvic::*;

impl RegisterExc for ::NucleoF746zg {
    fn register_exc(&self, exc: u8, handler: *mut HandleIrq) -> Result<IrqHandle, RegisterError> {
        Dispatcher::register_handler(IrqHandler::new(exc, handler)).ok_or(RegisterError::Unavailable)
    }
}

impl RegisterIrq for ::NucleoF746zg {
    fn register_irq(&self, irq: u8, handler: *mut HandleIrq) -> Result<IrqHandle, RegisterError> {
        if let Ok(handle) = Dispatcher::register_handler(IrqHandler::new(irq + 16, handler)).ok_or(RegisterError::Unavailable) {
            self.enable_irq(handle.irq);
        }
    }
}

impl EnableIrq for ::NucleoF746zg {
    fn enable_irq(&self, irq: u8) {
        NVIC.set_enabled(irq, true);
    }
}

impl DisableIrq for ::NucleoF746zg {
    fn disable_irq(&self, irq: u8) {
        NVIC.set_enabled(irq, false);
    }
}