pub use bobbin_sys::tick::{Tick, TICK, GetTick};

exception!(SYS_TICK, Tick::incr_ticks);

pub fn init() {
    use mcu::ext::systick::SystickHz;

    let reload_value = (::Clock::default().systick_hz() / 1000).as_u32() - 1;    
    ::mcu::STM32F74X.enable_tick(reload_value);    
}