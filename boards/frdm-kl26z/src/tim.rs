use hal::pit::*;
use hal::clock::Clock;
use clock::CLK;

pub const PIT_CH: usize = 0;
pub const PIT_RELOAD: u32 = 24_000;

pub fn init() {
    PIT.sim_enable();
    PIT.set_enabled(true);
}

pub fn delay(ms: u32) {
    PIT
        .set_load_value(PIT_CH, (PIT.clock(&CLK).unwrap() / 1000) * ms)
        .clr_interrupt_flag(PIT_CH)
        .set_timer_enabled(PIT_CH, true);
    while !PIT.interrupt_flag(PIT_CH) {}
    PIT.clr_interrupt_flag(PIT_CH);
}

// use hal::{sim, pit};

// pub fn tim0() -> pit::Timer {
//     sim::set_pit_enabled(true);
//     pit::set_enabled(true);
//     pit::timer(0)
// }

// pub fn tim0_unchecked() -> pit::Timer {
//     pit::timer(0)
// }

// pub fn delay(ms: u32) {
//     // Assume bus clock is 24Mhz
//     let t0 = tim0();
//     t0.set_load_value(24_000 * ms);
//     t0.clr_interrupt_flag();
//     t0.set_enabled(true);
//     while t0.interrupt_flag() == false {}
//     t0.clr_interrupt_flag();
// }