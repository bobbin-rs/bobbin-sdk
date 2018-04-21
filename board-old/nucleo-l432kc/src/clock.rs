// use chip::flash::FLASH;
pub use hal::clock::*;

pub const CLK: DynamicClock = DynamicClock {
    hse_osc: Some(8_000_000),
    lse_osc: Some(32767),
};

pub fn init() {
    init_pll()

}