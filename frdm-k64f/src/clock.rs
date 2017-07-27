use hal::clock::{self, ClockTree};

pub const CLK: ClockTree = ClockTree {
    xtal0: Some(50_000_000),
    xtal32: Some(32767),
};

pub fn init() {
    clock::init();
}