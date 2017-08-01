use hal::clock::{self, DynamicClock};

pub const CLK: DynamicClock = DynamicClock {
    xosc: None,
    xosc32k: Some(32767),
};

pub fn init() {
    clock::run_48mhz();    
}