use hal::clock::{self, DynamicClock};

pub const CLK: DynamicClock = DynamicClock {
    xtal0: Some(50_000_000),
    xtal32: Some(32767),
};

pub fn init() {
    clock::init();
}