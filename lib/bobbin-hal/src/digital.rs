//! Traits for simple digital input and output.

/// Read and update the current output state. 
pub trait DigitalOutput {
    /// Returns true if the output is high (true, 1).
    fn output(&self) -> bool;
    /// Set the output to be high (true, 1).
    fn set_output(&self, value: bool) -> &Self;
    /// Set the output to be the opposite of the current value.
    fn toggle_output(&self) -> &Self;
}

/// Read and update the current input state.
pub trait DigitalInput {
    /// Returns true if the input is high (true, 1).
    fn input(&self) -> bool;
}