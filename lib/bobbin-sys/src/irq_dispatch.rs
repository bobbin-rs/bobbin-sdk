use core::marker::PhantomData;
use core::ops::Deref;

pub enum Error {
    Unavailable
}

struct IrqToken;
static mut IRQ_TOKEN: Option<IrqToken> = Some(IrqToken);
static mut IRQ_HANDLERS: &'static mut [Option<IrqHandler>] = &mut [];

pub trait EnableDisableIrq {
    fn enable_irq(irq: u8);
    fn disable_irq(irq: u8);
}

pub trait HandleIrq {
    unsafe fn handle_irq(&self, irq: u8);
}

#[derive(Clone, Copy)]
pub struct IrqHandler {
    irq_num: u8,
    handler: *const HandleIrq,
}

impl IrqHandler {
    fn new(irq_num: u8, handler: *const HandleIrq) -> Self {
        Self { irq_num, handler }
    }
}

pub struct IrqDispatcher<T: EnableDisableIrq> {
    _phantom: PhantomData<T>,
}

impl<T: EnableDisableIrq> IrqDispatcher<T> {
    pub fn take() -> Self {
        unsafe { while let None = IRQ_TOKEN {} }
        IrqDispatcher { _phantom: PhantomData }
    }

    pub fn init(irq_handlers: &'static mut [Option<IrqHandler>]) -> Self {
        unsafe { while let None = IRQ_TOKEN {} }
        unsafe { IRQ_HANDLERS = irq_handlers }
        IrqDispatcher { _phantom: PhantomData }
    }

    pub fn release(_: IrqDispatcher<T>) {
        unsafe { IRQ_TOKEN = Some(IrqToken) }
    }

    #[inline]
    pub fn handlers() -> &'static mut [Option<IrqHandler>] {
        unsafe { IRQ_HANDLERS }
    }    

    fn irq_handlers(irq_num: u8) -> usize {
        let mut count = 0;
        for h in Self::handlers().iter() {
            if let &Some(h) = h {
                if h.irq_num == irq_num {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn register_handler<'h, H: 'static + HandleIrq>(&mut self, irq_num: u8, handler: &'h H) -> Result<Guard<'h, H>, Error> {        
        for h in Self::handlers().iter_mut() {
            if h.is_none() {
                *h = Some(IrqHandler::new(irq_num, handler));
                return Ok(Guard::new(handler))
            }
        }
        Err(Error::Unavailable)
    }

    fn unregister_handler(handler: *const u8) {
        for h in Self::handlers().iter_mut() {
            let mut clear = false;
            if let Some(h) = h {
                if h.handler as *const u8 == handler {
                    clear = true;
                }
            }
            if clear {
                *h = None;
            }
        }        
    }

    #[inline]
    pub unsafe fn dispatch(irq_num: u8) -> bool {
        let mut handled: bool = false;        
        for handler in Self::handlers() {
            if let Some(handler) = handler {
                if handler.irq_num == irq_num {
                    (*handler.handler).handle_irq(irq_num);
                    handled = true;
                }
            }
        }
        handled
    }    
}

#[must_use]
#[derive(Debug)]
pub struct Guard<'a, H: 'a> {
    handler: &'a H,
}

impl<'a, H: 'a> Guard<'a, H> {
    fn new(handler: &'a H) -> Self {
        Guard { handler }
    }
}

// impl<'a, H: 'a> Guard<'a, H> {
//     pub fn exc_num(&self) -> u8 {
//         self.exc_num
//     }

//     #[inline]
//     pub fn handlers() -> &'static mut [Option<ExceptionHandler>] {
//         unsafe {
//             ::core::slice::from_raw_parts_mut(EXC_HANDLERS_PTR, EXC_HANDLERS_LEN)
//         }        
//     }

//     pub fn slots_used_for_exc(exc_num: u8) -> usize {
//         let mut count = 0;
//         for h in Self::handlers().iter() {
//             if let &Some(h) = h {
//                 if h.exc_num == exc_num {
//                     count += 1;
//                 }
//             }
//         }
//         count
//     }
// }

impl<'a, H: 'a> Drop for Guard<'a, H> {
    fn drop(&mut self) {
        IrqDispatcher::unregister_handler(self.handler as *const H as *const u8)
    }
}

impl<'a, H: 'a> Deref for Guard<'a, H> {
    type Target = H;
    fn deref(&self) -> &H {
        self.handler
    }
}

// pub struct Dispatcher<T: Default + ExceptionHandlers>(T);

// impl<T: Default + ExceptionHandlers> Default for Dispatcher<T> {
//     fn default() -> Self {
//         Dispatcher(T::default())
//     }
// }

// impl<T: Default + ExceptionHandlers> Dispatcher<T> {
//     pub fn handle_exception() {
//         unsafe {
//             if !Self::dispatch(SCB.icsr().vectactive().value()) {
//                 // console::write_str("EXCEPTION\n");
//                 asm!("bkpt");
//                 loop {}
//             }
//         }
//     }    

//     pub fn slots() -> usize {
//         Self::handlers().len()
//     }

//     pub fn slots_used() -> usize {
//         Self::handlers().iter().filter(|h| h.is_some()).count()
//     }

//     pub fn slots_avail() -> usize {
//         Self::slots() - Self::slots_used()
//     }

//     #[inline]
//     pub fn handlers() -> &'static mut [Option<ExceptionHandler>] {
//         Self::require_slots();
//         unsafe {
//             ::core::slice::from_raw_parts_mut(EXC_HANDLERS_PTR, EXC_HANDLERS_LEN)
//         }
//     }
//     #[inline]
//     pub fn require_slots() {
//         unsafe {
//             if EXC_HANDLERS_PTR == ::core::ptr::null_mut() {
//                 let slots = T::exc_handlers();
//                 EXC_HANDLERS_PTR = slots.as_mut_ptr();
//                 EXC_HANDLERS_LEN = slots.len();
//             }
//         }
//     }
    
//     pub fn register_handler<'h, H: 'static + HandleException>(&mut self, exc_num: u8, handler: &'h H) -> Result<Guard<'h, H>, Error> {
//         let exc_handler = ExceptionHandler::new(exc_num, handler);
//         let exc_handlers = Self::handlers();
//         for i in 0..exc_handlers.len() {
//             if exc_handlers[i].is_none() {
//                 exc_handlers[i] = Some(exc_handler);
//                 match exc_handler.exc_num {
//                     15 => { SYSTICK.set_tick_interrupt(true); },
//                     e @ _ if e >= 16 => { NVIC.set_enabled(e- 16, true); },
//                     _ => {},
//                 }
//                 return Ok(Guard { exc_num: exc_num, index: i, handler})
//             }
//         }
//         Err(Error::Unavailable)
//     }

//     pub fn register_svcall_handler<'h, H: 'static + HandleException>(&mut self, handler: &'h H) -> Result<Guard<'h, H>, Error> {        
//         self.register_handler(11, handler)
//     }

//     pub fn register_pendsv_handler<'h,H: 'static + HandleException>(&mut self, handler: &'h H) -> Result<Guard<'h, H>, Error> {        
//         self.register_handler(14, handler)
//     }

//     pub fn register_systick_handler<'h,H: 'static + HandleException>(&mut self, handler: &'h H) -> Result<Guard<'h, H>, Error> {
//         self.register_handler(15, handler)
//     }

//     pub fn register_irq_handler<'h,H: 'static + HandleException>(&mut self, irq: u8, handler: &'h H) -> Result<Guard<'h, H>, Error> {        
//         self.register_handler(irq + 16, handler)
//     }

//     #[inline]
//     pub unsafe fn dispatch(exc_num: u8) -> bool {
//         let mut handled: bool = false;
//         let exc_handlers = Self::handlers();
//         for handler in exc_handlers.iter() {
//             if let Some(handler) = handler {
//                 if handler.exc_num == exc_num {
//                     (*handler.handler).handle_exception(exc_num);
//                     handled = true;
//                 }
//             }
//         }
//         handled
//     }
// }

// impl<T: Default + ExceptionHandlers> fmt::Debug for Dispatcher<T> {
//     fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
//         write!(out, "Dispatcher {{ slots: {} used: {} }}", Self::slots(), Self::slots_used())?;
//         Ok(())
//     }
// }