// use ::bobbin_common::sys::console;
use ::systick::SYSTICK;
use ::nvic::*;
use ::scb::*;

use core::ops::Deref;
use core::fmt;

macro_rules! impl_handlers {
    ($id:ident, $ty:ident, $len:expr) => {
        static mut $id: [Option<ExceptionHandler>; $len] = [None; $len];
        #[derive(Default)]
        pub struct $ty;
        impl ExceptionHandlers for $ty {
            fn exc_handlers() -> &'static mut [Option<ExceptionHandler>] { unsafe { &mut $id }}
        }
    }
}

impl_handlers!(EXC_HANDLERS_0, ExcHandlers0, 0);
impl_handlers!(EXC_HANDLERS_1, ExcHandlers1, 1);
impl_handlers!(EXC_HANDLERS_2, ExcHandlers2, 2);
impl_handlers!(EXC_HANDLERS_4, ExcHandlers4, 4);
impl_handlers!(EXC_HANDLERS_8, ExcHandlers8, 8);
impl_handlers!(EXC_HANDLERS_12, ExcHandlers12, 12);
impl_handlers!(EXC_HANDLERS_16, ExcHandlers16, 16);

pub trait ExceptionHandlers {
    fn exc_handlers() -> &'static mut [Option<ExceptionHandler>];
}

static mut EXC_HANDLERS_PTR: *mut Option<ExceptionHandler> = ::core::ptr::null_mut();
static mut EXC_HANDLERS_LEN: usize = 0;

#[derive(Debug)]
pub enum Error {
    Unavailable,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Exception {
    Reset,
    NMI,
    HardFault,
    MemManage,
    BusFault,
    UsageFault,
    SVCall,
    PendSV,
    SysTick,
    Interrupt(u8),
    Reserved(u8),
}

impl From<u8> for Exception {
    fn from(other: u8) -> Self {
        match other {
            1 => Exception::Reset,
            2 => Exception::NMI,
            3 => Exception::HardFault,
            4 => Exception::MemManage,
            5 => Exception::BusFault,
            6 => Exception::UsageFault,
            7 | 8 | 9 | 10 => Exception::Reserved(other),
            11 => Exception::SVCall,
            12 | 13 => Exception::Reserved(other),
            14 => Exception::PendSV,
            15 => Exception::SysTick,
            _ => Exception::Interrupt(other - 16),
        }
    }
}

impl From<Exception> for u8 {
    fn from(other: Exception) -> u8 {
        match other {
            Exception::Reset => 1,
            Exception::NMI => 2,
            Exception::HardFault => 3,
            Exception::MemManage => 4,
            Exception::BusFault => 5,
            Exception::UsageFault => 6,
            Exception::SVCall => 11,
            Exception::PendSV => 14,
            Exception::SysTick => 15,
            Exception::Interrupt(n) => n + 16,
            Exception::Reserved(n) => n,
        }
    }
}

pub trait HandleException {
    unsafe fn handle_exception(&self, exc: u8);
}

#[derive(Clone, Copy)]
pub struct ExceptionHandler {
    exc_num: u8,
    handler: *const HandleException,
}

impl ExceptionHandler {
    fn new(exc_num: u8, handler: *const HandleException) -> Self {
        Self { exc_num, handler }
    }
}

#[must_use]
#[derive(Debug)]
pub struct Guard<'a, H: 'a> {
    exc_num: u8,
    index: usize,
    handler: &'a H,
}

impl<'a, H: 'a> Guard<'a, H> {
    pub fn exc_num(&self) -> u8 {
        self.exc_num
    }

    #[inline]
    pub fn handlers() -> &'static mut [Option<ExceptionHandler>] {
        unsafe {
            ::core::slice::from_raw_parts_mut(EXC_HANDLERS_PTR, EXC_HANDLERS_LEN)
        }        
    }

    pub fn slots_used_for_exc(exc_num: u8) -> usize {
        let mut count = 0;
        for h in Self::handlers().iter() {
            if let &Some(h) = h {
                if h.exc_num == exc_num {
                    count += 1;
                }
            }
        }
        count
    }
}

impl<'a, H: 'a> Drop for Guard<'a, H> {
    fn drop(&mut self) {
        if Self::slots_used_for_exc(self.exc_num) <= 1 {
            match self.exc_num {
                15 => { SYSTICK.set_tick_interrupt(false); },
                e @ _ if e >= 16 => { NVIC.set_enabled(e, false); },
                _ => {},
            }
        }
        Self::handlers()[self.index] = None
    }
}

impl<'a, H: 'a> Deref for Guard<'a, H> {
    type Target = H;
    fn deref(&self) -> &H {
        self.handler
    }
}

pub struct Dispatcher<T: Default + ExceptionHandlers>(T);

impl<T: Default + ExceptionHandlers> Default for Dispatcher<T> {
    fn default() -> Self {
        Dispatcher(T::default())
    }
}

impl<T: Default + ExceptionHandlers> Dispatcher<T> {
    pub fn handle_exception() {
        unsafe {
            if !Self::dispatch(SCB.icsr().vectactive().value()) {
                // console::write_str("EXCEPTION\n");
                asm!("bkpt");
                loop {}
            }
        }
    }    

    pub fn slots() -> usize {
        Self::handlers().len()
    }

    pub fn slots_used() -> usize {
        Self::handlers().iter().filter(|h| h.is_some()).count()
    }

    pub fn slots_avail() -> usize {
        Self::slots() - Self::slots_used()
    }

    #[inline]
    pub fn handlers() -> &'static mut [Option<ExceptionHandler>] {
        Self::require_slots();
        unsafe {
            ::core::slice::from_raw_parts_mut(EXC_HANDLERS_PTR, EXC_HANDLERS_LEN)
        }
    }
    #[inline]
    pub fn require_slots() {
        unsafe {
            if EXC_HANDLERS_PTR == ::core::ptr::null_mut() {
                let slots = T::exc_handlers();
                EXC_HANDLERS_PTR = slots.as_mut_ptr();
                EXC_HANDLERS_LEN = slots.len();
            }
        }
    }
    
    pub fn register_handler<'h, H: 'static + HandleException>(&mut self, exc_num: u8, handler: &'h H) -> Result<Guard<'h, H>, Error> {
        let exc_handler = ExceptionHandler::new(exc_num, handler);
        let exc_handlers = Self::handlers();
        for i in 0..exc_handlers.len() {
            if exc_handlers[i].is_none() {
                exc_handlers[i] = Some(exc_handler);
                match exc_handler.exc_num {
                    15 => { SYSTICK.set_tick_interrupt(true); },
                    e @ _ if e >= 16 => { NVIC.set_enabled(e- 16, true); },
                    _ => {},
                }
                return Ok(Guard { exc_num: exc_num, index: i, handler})
            }
        }
        Err(Error::Unavailable)
    }

    pub fn register_svcall_handler<'h, H: 'static + HandleException>(&mut self, handler: &'h H) -> Result<Guard<'h, H>, Error> {        
        self.register_handler(11, handler)
    }

    pub fn register_pendsv_handler<'h,H: 'static + HandleException>(&mut self, handler: &'h H) -> Result<Guard<'h, H>, Error> {        
        self.register_handler(14, handler)
    }

    pub fn register_systick_handler<'h,H: 'static + HandleException>(&mut self, handler: &'h H) -> Result<Guard<'h, H>, Error> {
        self.register_handler(15, handler)
    }

    pub fn register_irq_handler<'h,H: 'static + HandleException>(&mut self, irq: u8, handler: &'h H) -> Result<Guard<'h, H>, Error> {        
        self.register_handler(irq + 16, handler)
    }

    #[inline]
    pub unsafe fn dispatch(exc_num: u8) -> bool {
        let mut handled: bool = false;
        let exc_handlers = Self::handlers();
        for handler in exc_handlers.iter() {
            if let Some(handler) = handler {
                if handler.exc_num == exc_num {
                    (*handler.handler).handle_exception(exc_num);
                    handled = true;
                }
            }
        }
        handled
    }
}

impl<T: Default + ExceptionHandlers> fmt::Debug for Dispatcher<T> {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "Dispatcher {{ slots: {} used: {} }}", Self::slots(), Self::slots_used())?;
        Ok(())
    }
}