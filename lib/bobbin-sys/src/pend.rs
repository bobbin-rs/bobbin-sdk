//! Global pend handler

use ::core::ptr;
use ::core::mem;
use ::core::slice;
use core::marker::PhantomData;

#[derive(Debug)]
pub enum Error {
    Timeout,
    NoHandlerSlots,
}

struct PendToken;
static mut PEND_TOKEN: Option<PendToken> = Some(PendToken);
static mut PEND_HANDLERS_PTR: *mut Option<*const HandlePend> = ptr::null_mut();
static mut PEND_HANDLERS_LEN: usize = 0;

/// Global singleton providing a pend task handler.
pub struct Pend {
    _private: ()
}

impl Pend {
    /// Acquire the global Pend singleton.
    pub fn take() -> Pend {
        unsafe { while let None = PEND_TOKEN {} }
        Pend { _private: () }
    }

    /// Release the global Pend singleton.
    pub fn release(_pend: Pend) {
        unsafe { PEND_TOKEN = Some(PendToken) }
    }

    pub fn init(ptr: *mut Option<*const HandlePend>, len: usize) -> Self {
        unsafe { 
            while let None = PEND_TOKEN {} 
            PEND_HANDLERS_PTR = ptr;
            PEND_HANDLERS_LEN = len;            
        }        
        Pend { _private: () }
    }

    #[inline]
    fn handlers() -> &'static mut [Option<*const HandlePend>] {
        unsafe {
            slice::from_raw_parts_mut(PEND_HANDLERS_PTR, PEND_HANDLERS_LEN)
        }       
    }

    pub fn register<'a>(&mut self, handler: &'a HandlePend) -> Result<PendGuard<'a>, Error> {
        let handler = unsafe { mem::transmute(handler as *const HandlePend) };
        for slot in Self::handlers().iter_mut() {
            if slot.is_none() {
                *slot = Some(handler);
                return Ok(PendGuard::new(handler))
            }
        }
        Err(Error::NoHandlerSlots)
    }    

    /// Run the global pend handler.
    #[inline]
    pub fn pend() {        
        for h in Self::handlers() {
            if let Some(h) = h {
                unsafe { (&*(*h)).handle_pend() }
            }
        }
    }
}

pub struct PendGuard<'a> {
    handler: *const HandlePend,
    _phantom: PhantomData<&'a HandlePend>
}

impl<'a> PendGuard<'a> {
    fn new(handler: *const HandlePend) -> Self {
        PendGuard { handler, _phantom: PhantomData }
    }
}

impl<'a> Drop for PendGuard<'a> {
    fn drop(&mut self) {
        for slot in Pend::handlers().iter_mut() {
            let mut remove = false;
            if let Some(handler) = slot {
                if self.handler == *handler {
                    remove = true;
                }
            }
            if remove {
                *slot = None;
            }
        }
    }
}

pub trait HandlePend {
    fn handle_pend(&self);
}
