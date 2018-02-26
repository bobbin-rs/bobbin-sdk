pub type Hz = Option<u32>;

pub const HZ: Hz = Some(1);
pub const KHZ: Hz = Some(1_000);
pub const MHZ: Hz = Some(1_000_000);
pub const GHZ: Hz = Some(1_000_000_000);

pub trait SystemClock {}

pub trait Clock<A> {
    fn clock(&self) -> Hz;
    fn clock_for(&self, _a: A) -> Hz {
        self.clock()
    }
}

pub trait Systick {
    fn systick(&self) -> Hz;
}

pub trait Millis {
    /// Returns the number of milliseconds modulo 2^32 since the clock was started.
    fn millis(&self) -> u32;
}
