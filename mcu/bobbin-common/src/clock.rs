pub use hz::Hz;

pub trait ClockTree {}

pub trait Clock: Default {
    fn hz() -> Hz;
    fn hz_of(&self) -> Hz { Self::hz() }
    fn value(&self) -> u32 { Self::hz().into() }
}

pub trait ClockFor<P> 
where 
    Self::Out : Clock
{
    type Out;
    fn clock_for(&self, _p: &P) -> Self::Out { Self::Out::default() }
}

pub trait Millis {
    /// Returns the number of milliseconds modulo 2^32 since the clock was started.
    fn millis(&self) -> u32;
}

pub trait ClockSource<T> {
    fn clock_source(&self) -> T;
    fn set_clock_source(&self, clk: T) -> &Self;
}