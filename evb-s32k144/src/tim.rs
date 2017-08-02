use hal::lpit::*;
use hal::pcc::ClockSource;
use clock::CLK;
use hal::clock::Clock;

pub const PIT: Lpit0 = LPIT0;
pub const PIT_CH: Lpit0Ch0 = LPIT0_CH0;
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
    PIT_CH.delay(ms, (PIT.clock(&CLK).unwrap() / 1000));
}
