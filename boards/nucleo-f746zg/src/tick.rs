pub use mcu::ext::ms_tick::*;

exception!(SYS_TICK, MsTick::handle_exception);

pub fn init() {
    use mcu::ext::systick::SystickHz;

    let reload_value = (::Clock::default().systick_hz() / 1000).as_u32() - 1;    
    MS_TICK.enable(reload_value);    
}