//! Global timer

use ::core::cell::UnsafeCell;
use ::core::ptr;

pub enum Error {
    Timeout
}

static mut TICKS: UnsafeCell<u32> = UnsafeCell::new(u32::max_value() - 2500);

struct TickToken;
static mut TICK_TOKEN: Option<TickToken> = Some(TickToken);

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

    /// Increment the global tick counter. Usually called from a timer interrupt.
    #[inline]
    pub fn tick() {
        unsafe { ptr::write_volatile(TICKS.get(), ptr::read_volatile(TICKS.get()).wrapping_add(1)) }
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

// pub trait GetTick {
//     fn enable_tick(&self, ms_hz: u32);
//     fn disable_tick(&self);
//     fn tick(&self) -> &Tick {
//         &TICK
//     }
// }

