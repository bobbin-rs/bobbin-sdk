static mut IRQ_HANDLERS_PTR: *mut Option<IrqHandler> = ::core::ptr::null_mut();
static mut IRQ_HANDLERS_LEN: usize = 0;

pub enum IrqError {
    Unhandled
}

pub trait HandleIrq {
    unsafe fn handle_irq(&mut self) -> Result<(), IrqError>;
}

#[derive(Clone, Copy)]
struct IrqHandler {
    irq: u8,
    handler: *mut HandleIrq,
}

impl IrqHandler {
    fn new(irq: u8, handler: *mut HandleIrq) -> Self {
        Self { irq, handler }
    }
}

#[must_use]
pub struct IrqHandle {
    irq: u8,
    index: usize,
}

impl Drop for IrqHandle {
    fn drop(&mut self) {
        Dispatcher::handlers()[self.index] = None
    }
}

pub struct Dispatcher {}

impl Dispatcher {
    fn init(slots: &'static mut [Option<IrqHandler>]) {
        unsafe {
            IRQ_HANDLERS_PTR = slots.as_mut_ptr();
            IRQ_HANDLERS_LEN = slots.len();
        }
    }

    fn slots() -> usize {
        Self::handlers().len()
    }

    fn slots_used() -> usize {
        Self::handlers().iter().filter(|h| h.is_some()).count()
    }

    #[inline]
    fn handlers() -> &'static mut [Option<IrqHandler>] {
        unsafe {
            core::slice::from_raw_parts_mut(IRQ_HANDLERS_PTR, IRQ_HANDLERS_LEN)
        }

    }
    
    fn register_handler(irq_handler: IrqHandler) -> Option<IrqHandle> {
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
    unsafe fn dispatch_irq(irq: u8) {
        let irq_handlers = Self::handlers();
        for i in 0..irq_handlers.len() {
            if let Some(handler) = irq_handlers[i] {                    
                if handler.irq == irq {
                    match (*handler.handler).handle_irq() {
                        Ok(_) => return,
                        Err(_) => {},
                    }
                }
            }
        }
    }
}

