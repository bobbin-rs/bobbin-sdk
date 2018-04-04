pub mod dispatch;

use self::dispatch::*;

use mcu::nvic::*;

static mut IRQ_HANDLERS_PTR: *mut Option<IrqHandler> = ::core::ptr::null_mut();
static mut IRQ_HANDLERS_LEN: usize = 0;

#[derive(Clone, Copy)]
pub struct IrqHandler {
    irq: u8,
    handler: *mut HandleIrq,
}

impl IrqHandler {
    pub fn new(irq: u8, handler: *mut HandleIrq) -> Self {
        Self { irq, handler }
    }

    pub fn irq(&self) -> u8 {
        self.irq
    }
}

#[must_use]
pub struct IrqHandle {
    pub irq: u8,
    index: usize,
}

impl IrqHandle {
    pub fn irq(&self) -> u8 {
        self.irq
    }    
}

impl Drop for IrqHandle {
    fn drop(&mut self) {
        if Dispatcher::slots_used_for_irq(self.irq) <= 1 {
            NVIC.set_enabled(self.irq, true);
        }
        Dispatcher::handlers()[self.index] = None
    }
}

pub struct Dispatcher {}

impl Dispatcher {
    pub fn init(slots: &'static mut [Option<IrqHandler>]) {
        unsafe {
            IRQ_HANDLERS_PTR = slots.as_mut_ptr();
            IRQ_HANDLERS_LEN = slots.len();
        }
    }

    pub fn slots() -> usize {
        Self::handlers().len()
    }

    pub fn slots_used() -> usize {
        Self::handlers().iter().filter(|h| h.is_some()).count()
    }

    pub fn slots_used_for_irq(irq: u8) -> usize {
        let mut count = 0;
        for h in Self::handlers().iter() {
            if let &Some(h) = h {
                if h.irq == irq {
                    count += 1;
                }
            }
        }
        count
    }

    #[inline]
    pub fn handlers() -> &'static mut [Option<IrqHandler>] {
        unsafe {
            ::core::slice::from_raw_parts_mut(IRQ_HANDLERS_PTR, IRQ_HANDLERS_LEN)
        }

    }
    
    pub fn register_handler(irq_handler: IrqHandler) -> Option<IrqHandle> {
        let irq_handlers = Self::handlers();
        for i in 0..irq_handlers.len() {
            if irq_handlers[i].is_none() {
                irq_handlers[i] = Some(irq_handler);
                if irq_handler.irq >= 16 {
                    NVIC.set_enabled(irq_handler.irq, true);
                }
                return Some(IrqHandle { irq: irq_handler.irq, index: i })
            }
        }
        None
    }

    #[inline]
    pub unsafe fn dispatch_irq(irq: u8) -> IrqResult {
        let mut handled: bool = false;
        let irq_handlers = Self::handlers();
        for i in 0..irq_handlers.len() {
            if let Some(handler) = irq_handlers[i] {                    
                if handler.irq == irq {
                    match (*handler.handler).handle_irq(irq) {
                        IrqResult::End => return IrqResult::End,
                        IrqResult::Continue => handled = true,
                    }
                }
            }
        }
        if handled {
            IrqResult::End
        } else {
            IrqResult::Continue
        }
    }
}



impl RegisterExc for ::NucleoF746zg {
    type Handle = IrqHandle;
    fn register_exc(&self, exc: u8, handler: *mut HandleIrq) -> Result<IrqHandle, RegisterError> {
        Dispatcher::register_handler(IrqHandler::new(exc, handler)).ok_or(RegisterError::Unavailable)
    }
}

impl RegisterIrq for ::NucleoF746zg {
    type Handle = IrqHandle;
    fn register_irq(&self, irq: u8, handler: *mut HandleIrq) -> Result<IrqHandle, RegisterError> {
        if let Ok(handle) = Dispatcher::register_handler(IrqHandler::new(irq + 16, handler)).ok_or(RegisterError::Unavailable) {
            self.enable_irq(handle.irq);
            Ok(handle)
        } else {
            Err(RegisterError::Unavailable)
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