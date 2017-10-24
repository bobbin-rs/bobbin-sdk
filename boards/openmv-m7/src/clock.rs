use hal::clock::{self, DynamicClock};

pub const CLK: DynamicClock = DynamicClock {
    hse_osc: Some(25_000_000),
    lse_osc: None,
};

pub fn init() {
    clock::enable_pll_hse_mode(25, 336, 2, 7);
}
