use hal::clock::{self, DynamicClock};

pub const CLK: DynamicClock = DynamicClock {
    xosc: Some(8_000_000),
    xrtc: None,
};

pub fn init() {
    clock::init();
}