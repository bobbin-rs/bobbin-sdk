use common::dispatch::*;
use mcu::nvic::*;

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

pub trait EnableIrq {
    fn enable_irq(&self, irq: u8) {
        NVIC.set_enabled(irq, true);
    }
}

pub trait DisableIrq {
    fn disable_irq(&self, irq: u8) {
        NVIC.set_enabled(irq, false);
    }
}