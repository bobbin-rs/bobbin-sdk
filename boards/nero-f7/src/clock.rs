use hal::clock::{self, DynamicClock};

pub const CLK: DynamicClock = DynamicClock {
    hse_osc: None,
    lse_osc: None,
};

pub fn init() {
    clock::enable_pll_hsi_mode();
}
