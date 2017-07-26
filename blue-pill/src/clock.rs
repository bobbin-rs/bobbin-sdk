use hal::clock::{self, ClockTree};

pub const CLK: ClockTree = ClockTree { hse_osc: Some(8_000_000) };

pub fn init() {
    clock::enable_pll_external_mode();
}