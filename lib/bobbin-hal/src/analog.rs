//! Traits abstracting reading and writing analog values (typically voltages)
//! and are implemented by ADC and DAC channels.
//! 
//! TODO: Add Error to trait

/// AnalogRead abstracts sampling and reading an ADC channel producing samples of type `T`.
/// Proper operation requires a `start()`, waiting for `is_complete()` to be true, and then
/// a `read()` of the sample, in that sequence.
pub trait AnalogRead<T> {
    /// Start sampling an ADC channel.
    fn start(&self) -> &Self;
    /// Returns true if the sampling process is complete.
    fn is_complete(&self) -> bool;
    /// Blocks actively until the sampling process is complete.
    fn wait_complete(&self) -> &Self {
        while !self.is_complete() {};
        self
    }
    /// Returns the result of the most recent sampling process. The return value is undefined
    /// if the ADC channel has not previously been started and completed.
    fn read(&self) -> T;
    /// Returns the result of a complete sampling cycle, after starting a sample and waiting
    /// for the sample to complete. The return value is undefined if called while a previous
    /// sample process is in progress.
    fn analog_read(&self) -> T {
        self.start().wait_complete().read()
    }
}

/// AnalogWrite abstracts writing a DAC channel with values of type `T`.
pub trait AnalogWrite<T> {
    /// Configures the DAC channel to output a voltage proportional to `value`. The exact output
    /// voltage as well as the propagation delay is implementation defined.
    fn analog_write(&self, value: T) -> &Self;    
}