use hal::clock::{self, DynamicClock};

pub const CLK: DynamicClock = DynamicClock {
    hse_osc: Some(8_000_000),
    lse_osc: Some(32768),
};

pub fn init() {
    clock::enable_pll_external_mode();
}