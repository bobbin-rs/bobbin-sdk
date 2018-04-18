pub use mcu::ext::ms_tick::*;

exception!(SYS_TICK, MsTick::handle_exception);

pub fn init() {
    use mcu::systick::SYSTICK;
    use mcu::ext::systick::{SystickHz, ClockSource};

    let reload_value = (::Clock::default().systick_hz() / 1000).as_u32() - 1;  
    SYSTICK.set_clock_source(ClockSource::Internal);    
  
    MS_TICK.enable(reload_value);    
}