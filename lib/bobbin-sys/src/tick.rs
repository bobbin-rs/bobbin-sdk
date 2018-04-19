use ::core::cell::UnsafeCell;
use ::core::ptr;

pub enum Error {
    Timeout
}

static mut TICKS: UnsafeCell<u32> = UnsafeCell::new(u32::max_value() - 2500);

pub const TICK: Tick = Tick;
#[derive(Default)]
pub struct Tick;

impl Tick {
    #[inline]
    pub fn incr_ticks() {
        unsafe { ptr::write_volatile(TICKS.get(), ptr::read_volatile(TICKS.get()).wrapping_add(1)) }
    }

    #[inline]
    pub fn ticks(&self) -> u32 {
        unsafe { ptr::read_volatile(TICKS.get()) }
    }

    #[inline]
    pub fn ticks_since(&self, t: u32) -> u32 {
        self.ticks().wrapping_sub(t)
    }

    pub fn delay(&self, ms: u32) {
        let t = self.ticks();
        while self.ticks_since(t) < ms {}
    }

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

pub trait GetTick {
    fn enable_tick(&self, ms_hz: u32);
    fn disable_tick(&self);
    fn tick(&self) -> &Tick {
        &TICK
    }
}

