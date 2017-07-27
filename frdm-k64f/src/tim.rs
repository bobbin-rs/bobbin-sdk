use hal::pit::*;
use clock::CLK;
pub const PIT_CH: usize = 0;
pub const PIT_RELOAD: u32 = 60000;

pub fn init() {
    PIT.sim_enable();
    PIT.set_enabled(true);
}

pub fn delay(ms: u32) {
    PIT
        .set_load_value(PIT_CH, (CLK.clock(&PIT).expect("No bus clock") / 1000) * ms)
        .clr_interrupt_flag(PIT_CH)
        .set_timer_enabled(PIT_CH, true);
    while !PIT.interrupt_flag(PIT_CH) {}
    PIT.clr_interrupt_flag(PIT_CH);
}