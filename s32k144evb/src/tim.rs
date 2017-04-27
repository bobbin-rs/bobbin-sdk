use hal::{sim, pit};

pub fn tim0() -> pit::Timer {
    sim::set_pit_enabled(true);
    pit::set_enabled(true);
    pit::timer(0)
}

pub fn tim0_unchecked() -> pit::Timer {
    pit::timer(0)
}

pub fn delay(ms: u32) {
    // Assume bus clock is 60Mhz
    let t0 = tim0();
    t0.set_load_value(60_000 * ms);
    t0.clr_interrupt_flag();
    t0.set_enabled(true);
    while t0.interrupt_flag() == false {}
    t0.clr_interrupt_flag();
}