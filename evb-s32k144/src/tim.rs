use hal::lpit::*;
use hal::pcc::ClockSource;
use clock::CLK;
use hal::clock::Clock;

pub const PIT: Lpit0 = LPIT0;
pub const PIT_CH: usize = 0;
pub const PIT_CLK: ClockSource = ClockSource::SPLLDIV2;
pub const PIT_RELOAD: u32 = 40_000;

pub fn init() {
    // Clock source must be set before enabling clock
    PIT.pcc_set_enabled(false);
    PIT.pcc_set_clock_source(PIT_CLK);
    PIT.pcc_set_enabled(true);
    PIT.set_enabled(true);
}

pub fn delay(ms: u32) {
    PIT.set_ch_value(PIT_CH, (PIT.clock(&CLK).unwrap() / 1000) * ms);
    PIT.clr_ch_tif(PIT_CH);
    PIT.set_ch_enabled(PIT_CH, true);
    
    while !PIT.ch_tif(PIT_CH) {}

    PIT
        .clr_ch_tif(PIT_CH)
        .set_ch_enabled(PIT_CH, false);
}

// use hal::{pcc, lpit};
// use core::ptr;

// pub fn lpit0() -> lpit::Timer {
//     pcc::set_lpit_enabled(true, pcc::Source::SPLLDIV2);
//     lpit::timer(lpit::LPIT0, 0)
// }

// pub fn lpit1() -> lpit::Timer {
//     pcc::set_lpit_enabled(true, pcc::Source::SPLLDIV2);
//     lpit::timer(lpit::LPIT0, 1)
// }

// static mut DELAY_DONE: bool = false;
// pub fn delay(ms: u32) {
//     unsafe { DELAY_DONE = false; }
//     unsafe extern "C" fn delay_handler() {
//         let t = lpit::timer(lpit::LPIT0, 0);
//         if t.tif() {
//             t.clr_tif();
//             DELAY_DONE = true;
//         }
//     }

//     // Assume clock is 40Mhz
//     let t0 = lpit0();
//     t0.set_value(40_000 * ms);
//     t0.set_handler(Some(delay_handler));
//     t0.clr_tif();
//     t0.set_tie(true);
//     t0.set_doze_enabled(true);
//     t0.set_enabled(true);    
    
//     unsafe {
//         while !ptr::read_volatile(&DELAY_DONE as *const bool) { asm!("wfi") }
//     }
//     t0.set_enabled(false);
//     t0.set_handler(None);
// }