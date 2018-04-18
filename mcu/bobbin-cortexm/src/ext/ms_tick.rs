use ::systick::SYSTICK;

use ::core::cell::UnsafeCell;
use ::core::ptr;

pub enum Error {
    Timeout
}

static mut MS_TICKS: UnsafeCell<u32> = UnsafeCell::new(u32::max_value() - 2500);

pub const MS_TICK: MsTick = MsTick;
pub struct MsTick;

impl MsTick {
    pub fn enable(&mut self, reload_value: u32) {
        let st = SYSTICK;
        st.set_reload_value(reload_value);
        st.set_current_value(reload_value);
        st.set_enabled(true);
        st.set_tick_interrupt(true);
    }

    pub fn disable(&mut self) {
        let st = SYSTICK;
        st.set_tick_interrupt(false);
        st.set_enabled(false);
    }

    #[inline]
    pub fn counter(&self) -> u32 {
        unsafe { ptr::read_volatile(MS_TICKS.get()) }
    }

    #[inline]
    fn incr_counter(&self) {
        unsafe { ptr::write_volatile(MS_TICKS.get(), self.counter().wrapping_add(1)) }
    }

    #[inline]
    pub fn ticks_since(&self, t: u32) -> u32 {
        self.counter().wrapping_sub(t)
    }

    pub fn delay(&self, ms: u32) {
        let t = self.counter();
        while self.ticks_since(t) < ms {}
    }

    pub fn wait_while<F: FnMut()->bool>(&self, timeout: u32, mut f: F) -> Result<(), Error> {
        let t = self.counter();
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
        let t = self.counter();
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

    pub fn handle_exception() {
        MS_TICK.incr_counter();
    }    
}

impl ::bobbin_hal::delay::Delay for MsTick {
    fn delay_ms(&self, ms: u32) {
        self.delay(ms)
    }
}

