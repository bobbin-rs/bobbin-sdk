//! Traits for performing simple delay operations.

/// Provides a simple millisecond blocking delay. See the implementer
/// documentation for details on granularity and minimum / maximium delay
/// times.
pub trait Delay {
    fn delay_ms(&self, ms: u32);
}