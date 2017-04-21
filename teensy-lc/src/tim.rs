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
    // Assume TIM0 clock is 24Mhz
    let t0 = tim0();
    t0.set_load_value(12_000 * ms);
    t0.clr_interrupt_flag();
    t0.set_enabled(true);
    while t0.interrupt_flag() == false {}
    t0.clr_interrupt_flag();
}