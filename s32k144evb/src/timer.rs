use hal::{pcc, lpit};

pub fn lpit0() -> lpit::Timer {
    pcc::set_lpit_enabled(true, pcc::Source::SPLLDIV2);
    lpit::timer(lpit::LPIT0, 0)
}

// use hal::{sim, lpit};

// pub fn tim0() -> pit::Timer {
//     sim::set_pit_enabled(true);
//     pit::set_enabled(true);
//     pit::timer(0)
// }

// pub fn tim0_unchecked() -> pit::Timer {
//     pit::timer(0)
// }

pub fn delay(ms: u32) {
    // Assume clock is 40Mhz
    let t0 = lpit0();
    t0.set_value(40_000 * ms);
    t0.clr_tif();
    t0.set_enabled(true);
    while t0.tif() == false {}
    t0.clr_tif();
    t0.set_enabled(false);
//     let t0 = tim0();
//     t0.set_load_value(60_000 * ms);
//     t0.clr_interrupt_flag();
//     t0.set_enabled(true);
//     while t0.interrupt_flag() == false {}
//     t0.clr_interrupt_flag();
}