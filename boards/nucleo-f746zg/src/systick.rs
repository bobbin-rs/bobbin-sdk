use ::core::cell::UnsafeCell;
use ::core::ptr;

static mut SYSTICK_COUNTER: UnsafeCell<u32> = UnsafeCell::new(0);

pub const SYSTICK: Systick = Systick;
pub struct Systick;

impl Systick {
    pub fn systick(&self) -> ::mcu::systick::Systick {
        ::mcu::systick::SYSTICK
    }
    pub fn enable(&mut self, reload_value: u32) {
        let st = self.systick();
        st.set_reload_value(reload_value);
        st.set_current_value(reload_value);
        st.set_enabled(true);
        st.set_tick_interrupt(true);
    }

    pub fn disable(&mut self) {
        let st = self.systick();
        st.set_tick_interrupt(false);
        st.set_enabled(false);
    }

    pub fn counter(&self) -> u32 {
        unsafe { ptr::read_volatile(SYSTICK_COUNTER.get()) }
    }

    fn incr_counter(&self) {
        unsafe { ptr::write_volatile(SYSTICK_COUNTER.get(), self.counter().wrapping_add(1)) }
    }

    pub fn delay(&self, ms: u32) {
        let deadline = self.counter().wrapping_add(ms);
        while self.counter() != deadline {}        
    }

    fn handle_exception() {
        SYSTICK.incr_counter();
    }    
}

impl ::bobbin_hal::delay::Delay for Systick {
    fn delay_ms(&self, ms: u32) {
        self.delay(ms)
    }
}

exception!(SYS_TICK, Systick::handle_exception);

pub fn init() {
    use mcu::ext::systick::SystickHz;

    let reload_value = (::Clock::default().systick_hz() / 1000).as_u32() - 1;    
    SYSTICK.enable(reload_value);    
}