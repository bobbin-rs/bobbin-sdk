//! Global timer

use ::core::cell::UnsafeCell;
use ::core::ptr;
use ::core::mem;
use ::core::slice;
use core::marker::PhantomData;

#[derive(Debug)]
pub enum Error {
    Timeout,
    NoHandlerSlots,
}

static mut TICKS: UnsafeCell<u32> = UnsafeCell::new(u32::max_value() - 2500);

struct TickToken;
static mut TICK_TOKEN: Option<TickToken> = Some(TickToken);
static mut TICK_HANDLERS_PTR: *mut Option<*const HandleTick> = ptr::null_mut();
static mut TICK_HANDLERS_LEN: usize = 0;

/// Global singleton providing a millisecond tick counter.
pub struct Tick {
    _private: ()
}

impl Tick {
    /// Acquire the global Tick singleton.
    pub fn take() -> Tick {
        unsafe { while let None = TICK_TOKEN {} }
        Tick { _private: () }
    }

    /// Release the global Tick singleton.
    pub fn release(_tick: Tick) {
        unsafe { TICK_TOKEN = Some(TickToken) }
    }

    pub fn init(ptr: *mut Option<*const HandleTick>, len: usize) -> Self {
        unsafe { 
            while let None = TICK_TOKEN {} 
            TICK_HANDLERS_PTR = ptr;
            TICK_HANDLERS_LEN = len;            
        }        
        Tick { _private: () }
    }

    #[inline]
    fn handlers() -> &'static mut [Option<*const HandleTick>] {
        unsafe {
            slice::from_raw_parts_mut(TICK_HANDLERS_PTR, TICK_HANDLERS_LEN)
        }       
    }

    pub fn register<'a>(&mut self, handler: &'a HandleTick) -> Result<TickGuard<'a>, Error> {
        let handler = unsafe { mem::transmute(handler as *const HandleTick) };
        for slot in Self::handlers().iter_mut() {
            if slot.is_none() {
                *slot = Some(handler);
                return Ok(TickGuard::new(handler))
            }
        }
        Err(Error::NoHandlerSlots)
    }    

    /// Increment the global tick counter. Usually called from a timer interrupt.
    #[inline]
    pub fn tick() {        
        let ticks = unsafe { 
            let ticks = ptr::read_volatile(TICKS.get()).wrapping_add(1);
            ptr::write_volatile(TICKS.get(), ticks);
            ticks
        };
        for h in Self::handlers() {
            if let Some(h) = h {
                unsafe { (&*(*h)).handle_tick(ticks) }
            }
        }
    }

    /// Returns the value of the global tick counter.
    #[inline]
    pub fn ticks(&self) -> u32 {
        unsafe { ptr::read_volatile(TICKS.get()) }
    }

    /// Returns the number of ticks elapsed since `t`.
    #[inline]
    pub fn ticks_since(&self, t: u32) -> u32 {
        self.ticks().wrapping_sub(t)
    }

    /// Busy wait until `ms` ticks have elapsed.
    pub fn delay(&self, ms: u32) {
        let t = self.ticks();
        while self.ticks_since(t) < ms {}
    }

    /// Loop until either `f` returns false or `timeout` ticks have elapsed. Returns
    /// Error::Timeout if the timeout has elapsed.
    pub fn wait_while<F: FnMut()->bool>(&self, timeout: u32, mut f: F) -> Result<(), Error> {
        let t = self.ticks();
        loop {
            if self.ticks_since(t) < timeout {
                if !f() {
                    return Ok(())
                }
            } else {
                return Err(Error::Timeout)
            }
        }        
    }

    /// Loop until either `f` returns true or `timeout` ticks have elapsed. Returns
    /// Error::Timeout if the timeout has elapsed.
    pub fn wait_until<F: FnMut()->bool>(&self, timeout: u32, mut f: F) -> Result<(), Error> {
        let t = self.ticks();
        loop {
            if self.ticks_since(t) < timeout {
                if f() {
                    return Ok(())
                }
            } else {
                return Err(Error::Timeout)
            }
        }        
    }
}

impl ::bobbin_hal::delay::Delay for Tick {
    fn delay_ms(&self, ms: u32) {
        self.delay(ms)
    }
}

impl<'a> ::bobbin_hal::delay::Delay for &'a Tick {
    fn delay_ms(&self, ms: u32) {
        self.delay(ms)
    }
}

pub struct TickGuard<'a> {
    handler: *const HandleTick,
    _phantom: PhantomData<&'a HandleTick>
}

impl<'a> TickGuard<'a> {
    fn new(handler: *const HandleTick) -> Self {
        TickGuard { handler, _phantom: PhantomData }
    }
}

impl<'a> Drop for TickGuard<'a> {
    fn drop(&mut self) {        
        for slot in Tick::handlers().iter_mut() {
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

pub trait HandleTick {
    fn handle_tick(&self, counter: u32);
}
