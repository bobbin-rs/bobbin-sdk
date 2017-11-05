use hal::clock::{self, DynamicClock};

pub const CLK: DynamicClock = DynamicClock {
    hse_osc: Some(8_000_000),
    lse_osc: Some(32768),
};

pub fn init() {
    // 8 MHz External Clock
    // VCO = 432MHz
    // PLL = 216MHz
    // PLLQ = 48MHz
    // SYSCLK = 216MHz
    // AHB = 216MHz
    // APB1 = 54MHz
    // APB2 = 108MHz

    clock::enable_pll_hse_bypass_mode(8, 432, 0b00, 9);
}