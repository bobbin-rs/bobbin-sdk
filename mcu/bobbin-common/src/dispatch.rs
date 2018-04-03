static mut IRQ_HANDLERS_PTR: *mut Option<IrqHandler> = ::core::ptr::null_mut();
static mut IRQ_HANDLERS_LEN: usize = 0;

pub enum IrqResult {
    End,
    Continue,
}

pub trait HandleIrq {
    unsafe fn handle_irq(&mut self, irq: u8) -> IrqResult;
}

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
    irq: u8,
    index: usize,
}

impl IrqHandle {
    pub fn irq(&self) -> u8 {
        self.irq
    }    
}

impl Drop for IrqHandle {
    fn drop(&mut self) {
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

