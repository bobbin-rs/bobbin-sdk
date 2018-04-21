//! Traits used for modeling MCU clock networks.

pub use hz::Hz;

/// Marker trait indicating the item is a Clock Tree.
pub trait ClockTree {}

/// Trait indicating the item is a clock with output frequency `hz()`.
pub trait Clock : Default {
    fn hz() -> Hz;
}

/// Trait allowing access to the clock for type `T`.
pub trait ClockFor<T> {
    fn clock_for(&self, T) -> Hz;    
}

/// Return the number of milliseconds since the clock was started.
pub trait Millis {
    /// Returns the number of milliseconds modulo 2^32 since the clock was started.
    fn millis(&self) -> u32;
}

/// Query and select the clock source for T.
pub trait ClockSource<T> {
    fn clock_source(&self) -> T;
    fn set_clock_source(&self, clk: T) -> &Self;
}
