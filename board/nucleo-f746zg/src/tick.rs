use mcu::systick::SYSTICK;
use mcu::ext::systick::SystickHz;
use bobbin_sys::tick::Tick;
use Clk;

exception!(SYS_TICK, Tick::incr_ticks);

pub fn init() {
    

    let ms_hz = (Clk::default().systick_hz() / 1000).as_u32() - 1;    
    let st = SYSTICK;
    st.set_reload_value(ms_hz);
    st.set_current_value(ms_hz);
    st.set_enabled(true);
    st.set_tick_interrupt(true);   
}