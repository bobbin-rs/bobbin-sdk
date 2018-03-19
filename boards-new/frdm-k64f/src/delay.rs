use clock::*;
use mcu::pit::*;

pub const PIT_CH: usize = 0;
pub const PIT_RELOAD: u32 = 60000;

pub fn init() {
    PIT.gate_enable();
    PIT.set_enabled(true);
}

pub fn delay(ms: u32) {
    let pit_clk = tree().u32_for(PIT);
    PIT
        // .set_load_value(PIT_CH, (PIT.clock(&CLK).expect("No bus clock") / 1000) * ms)
        .set_load_value(PIT_CH, (pit_clk / 1000) * ms)
        .clr_interrupt_flag(PIT_CH)
        .set_timer_enabled(PIT_CH, true);
    while !PIT.interrupt_flag(PIT_CH) {}
    PIT.clr_interrupt_flag(PIT_CH);
}