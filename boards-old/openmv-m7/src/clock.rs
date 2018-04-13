use hal::clock::{self, DynamicClock};

pub const CLK: DynamicClock = DynamicClock {
    hse_osc: Some(12_000_000),
    lse_osc: None,
};

pub fn init() {
    // 12 MHz External Clock
    // VCO = 432MHz
    // PLL = 216MHz
    // PLLQ = 48MHz
    // SYSCLK = 216MHz
    // AHB = 216MHz
    // APB1 = 54MHz
    // APB2 = 108MHz

    clock::enable_pll_hse_mode(12, 432, 0b00, 9);
}
