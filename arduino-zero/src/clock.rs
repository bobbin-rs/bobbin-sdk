use hal::clock::{self, ClockTree};

pub const CLK: ClockTree = ClockTree {
    xosc: None,
    xosc32k: Some(32767),
};

pub fn init() {
    clock::run_48mhz();    
}