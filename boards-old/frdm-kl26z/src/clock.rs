use hal::clock::{self, DynamicClock};

pub const CLK: DynamicClock = DynamicClock {
    xtal0: Some(8_000_000)
};

pub fn init() {
    clock::init();
}