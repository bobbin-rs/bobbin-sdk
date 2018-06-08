//! Interrupt dispatcher
//! 
//! This module implements a global singleton that operates as a interrupt
//! dispatcher supporting dynamic registration and unregistration of
//! interrupt handlers. 
//! 
//! Interrupt handlers implement the `HandleIrq` trait and then are registered
//! using `register_handler()`, which returns a guard object. The handler may
//! be accessed using the guard object, which automatically unregisters the handler
//! when it is dropped.
//! 
//! Multiple handlers can be registered for a single interrupt. An interrupt is automatically
//! enabled as soon as a handler is registered and is disabled as soon as the 
//! last handler has been unregistered.
//! 
//! The global singleton is designed to be initialized by the [System](../system/index.html)
//! and most operations are not available in Run mode.

use bobbin_mcu::mcu::Mcu;
use bobbin_mcu::irq::{Irq, IrqType, IrqMain, IRQ_MAIN};

use core::ops::Deref;
use core::ptr;
use core::slice;
use core::fmt;
use core::mem;
use core::marker::PhantomData;

/// Error returned by the interrupt dispatcher.
#[derive(Debug)]
pub enum Error {
    /// The dispatcher was unable to register the handler for the interrupt.
    IrqUnavailable(u8)
}

struct IrqToken;
static mut IRQ_TOKEN: Option<IrqToken> = Some(IrqToken);
static mut IRQ_HANDLERS_PTR: *mut Option<IrqHandler> = ptr::null_mut();
static mut IRQ_HANDLERS_LEN: usize = 0;

// pub trait EnableDisableIrq {
//     fn enable_irq(&self, irq: u8);
//     fn disable_irq(&self, irq: u8);
// }

/// Handle an interrupt.
pub trait HandleIrq : Sync {
    /// Handle an interrupt. The interrupt number is provided for handlers that support
    /// more than interrupt.
    fn handle_irq(&self, irq: u8);
}

#[derive(Clone, Copy)]
pub struct IrqHandler {
    irq_num: u8,
    handler: *const HandleIrq,
}

impl IrqHandler {
    fn new(irq_num: u8, handler: *const HandleIrq, ) -> Self {
        Self { irq_num, handler }
    }
}

/// The interrupt dispatcher singleton.
pub struct IrqDispatcher<MCU: Mcu> {
    _phantom: PhantomData<MCU>,
}

impl<MCU: Mcu> IrqDispatcher<MCU> {
    /// Acquire the global interrupt dispatcher singleton.
    pub fn take() -> Self {
        unsafe { while let None = IRQ_TOKEN {} }
        IrqDispatcher { _phantom: PhantomData }
    }

    /// Initialize the global interrupt dispatcher singleton with
    /// a list of IrqHandlers. 
    /// 
    /// NOTE: should be unsafe or accept &'static instead.
    pub fn init(ptr: *mut Option<IrqHandler>, len: usize) -> Self {
        unsafe { 
            while let None = IRQ_TOKEN {} 
            IRQ_HANDLERS_PTR = ptr;
            IRQ_HANDLERS_LEN = len;            
        }        
        IrqDispatcher { _phantom: PhantomData }
    }

    /// Release the global interrupt dispatcher.
    pub fn release(_: Self) {
        unsafe { IRQ_TOKEN = Some(IrqToken) }
    }

    pub fn handle_exception(exc: i16) {
        if exc > 16 && Self::dispatch(exc.wrapping_sub(16) as u8) {
            return
        } else {
            // ::bobbin_sys::console::write(b"Unhandled Exception: 0x");
            // ::bobbin_sys::console::write_u8_hex(exc);
            // ::bobbin_sys::console::write(b"\r\n");
            unsafe { asm!("bkpt") };
            loop {}
        }
    }

    /// Returns a reference to the list of interrupt handlers.
    /// 
    /// NOTE: should be private?
    #[inline]
    pub fn handlers() -> &'static mut [Option<IrqHandler>] {
        unsafe {
            slice::from_raw_parts_mut(IRQ_HANDLERS_PTR, IRQ_HANDLERS_LEN)
        }       
    }    

    /// Enable interrupt `irq_num`.
    pub fn enable_irq(irq_num: u8) {
        MCU::irq_enable(irq_num);
    }

    /// Disable interrupt `irq_num`.
    pub fn disable_irq(irq_num: u8) {
        MCU::irq_disable(irq_num);
    }

    /// Returns the total number of handler slots.
    pub fn slots() -> usize {
        Self::handlers().iter().count()
    }

    /// Returns the number of handler slots used.
    pub fn slots_used() -> usize {
        Self::handlers().iter().filter(|h| h.is_some()).count()
    }

    /// Returns the number of handler slots used by handlers for interrupt number `irq_num`.
    pub fn slots_for_irq(irq_num: u8) -> usize {
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

    /// Register a handler for interrupt number `irq_num`. The handler is automatically unregistered
    /// when the guard is dropped.
    pub fn register_handler<'h, H: 'h + HandleIrq>(&mut self, irq_num: u8, handler: &'h H) -> Result<Guard<'h, H, MCU>, Error> {        
        for h in Self::handlers().iter_mut() {
            if h.is_none() {
                *h = Some(IrqHandler::new(irq_num, unsafe { mem::transmute(handler as *const HandleIrq) }));
                Self::enable_irq(irq_num);
                return Ok(Guard::new(handler))
            }
        }
        Err(Error::IrqUnavailable(irq_num))
    }

    /// Register a handler for interrupt type `irq_type` of peripheral `periph`. The handler is automatically unregistered
    /// when the guard is dropped.
    pub fn register_periph_type_handler<'h, P: Irq<I>, I: IrqType, H: 'h + HandleIrq>(&mut self, periph: P, irq_type: I, handler: &'h H) -> Result<Guard<'h, H, MCU>, Error> {        
        let irq_num = periph.irq_number_for(irq_type);
        self.register_handler(irq_num, handler)
    }

    /// Register a handler for the main interrupt handler for `periph`. The handler is automatically unregistered
    /// when the guard is dropped.
    pub fn register_periph_handler<'h, P: Irq<IrqMain>, H: 'h + HandleIrq>(&mut self, periph: P, handler: &'h H) -> Result<Guard<'h, H, MCU>, Error> {        
        self.register_periph_type_handler(periph, IRQ_MAIN, handler)
    }

    fn unregister_handler(handler: *const u8) {
        for h in Self::handlers().iter_mut() {
            let mut clear = false;
            if let Some(h) = h {
                if h.handler as *const u8 == handler {
                    clear = true;
                    if Self::slots_for_irq(h.irq_num) == 1 {
                        Self::disable_irq(h.irq_num);
                    }
                }
            }
            if clear {
                *h = None;
            }
        }        
    }

    /// Execute all interrupt handlers registered for `irq_num`. Returns
    /// `true` if the interrupt has been handled by at least one handler,
    /// `false` if the interrupt was not handled.
    #[inline]
    pub fn dispatch(irq_num: u8) -> bool {
        let mut handled: bool = false;        
        for handler in Self::handlers() {
            if let Some(handler) = handler {
                if handler.irq_num == irq_num {
                    unsafe { (*handler.handler).handle_irq(irq_num) };
                    handled = true;
                }
            }
        }
        handled
    }    
}

/// A handle to a registered interrupt handler. Provides access to the interrupt handler and
/// automatically unregisters the handler when dropped.
#[must_use]
#[derive(Debug)]
pub struct Guard<'a, H: 'a, MCU: Mcu> {
    handler: &'a H,
    _phantom: PhantomData<MCU>,
}

impl<'a, H: 'a, MCU: Mcu> Guard<'a, H, MCU> {
    fn new(handler: &'a H) -> Self {
        Guard { handler, _phantom: PhantomData }
    }
}

impl<'a, H: 'a, MCU: Mcu> Drop for Guard<'a, H, MCU> {
    fn drop(&mut self) {
        IrqDispatcher::<MCU>::unregister_handler(self.handler as *const H as *const u8)
    }
}

impl<'a, H: 'a, MCU: Mcu> Deref for Guard<'a, H, MCU> {
    type Target = H;
    fn deref(&self) -> &H {
        self.handler
    }
}

impl<MCU: Mcu> fmt::Debug for IrqDispatcher<MCU> {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "IrqDispatcher {{ slots: {} used: {} }}", Self::slots(), Self::slots_used())?;
        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use core::cell::Cell;

    struct Driver {
        count: Cell<usize>
    }

    impl Driver {
        fn incr(&self) {
            self.count.set(self.count.get() + 1)
        }

        fn count(&self) -> usize {
            self.count.get()
        }
    }

    impl HandleIrq for Driver {
        fn handle_irq(&self, _irq: u8) {
            self.incr();
        }
    }

    unsafe impl Sync for Driver {}


    #[test]
    fn test_dispatcher() {
        static mut HANDLERS: [Option<IrqHandler>; 4] =[None; 4];
        static mut IRQ_MGR: u8 = 0;

        fn enable_disable(irq: u8, value: bool) {
            unsafe { 
                if value {
                    IRQ_MGR |= 1 << irq;
                } else {
                    IRQ_MGR &= !(1 << irq);
                }
            }
        }
        IrqDispatcher::set_enable_disable(enable_disable);
        let mut irq_d = unsafe {
            
            IrqDispatcher::init(HANDLERS.as_mut_ptr(), HANDLERS.len())            
        };
        assert_eq!(IrqDispatcher::slots(), 4);
        let d = Driver { count: Cell::new(0) };
        assert_eq!(d.count(), 0);
        {
            let g = irq_d.register_handler(1, &d).unwrap();
            assert_eq!(IrqDispatcher::slots_used(), 1);
            unsafe { assert_eq!(IRQ_MGR, 1 << 1); }


            assert_eq!(IrqDispatcher::dispatch(1), true);
            assert_eq!(g.count(), 1);
            assert_eq!(IrqDispatcher::dispatch(2), false);
            assert_eq!(g.count(), 1);
            assert_eq!(IrqDispatcher::dispatch(1), true);
            assert_eq!(g.count(), 2);
        }
        assert_eq!(IrqDispatcher::slots_used(), 0);
        unsafe { assert_eq!(IRQ_MGR, 0); }
        assert_eq!(d.count(), 2);
        
    }
}