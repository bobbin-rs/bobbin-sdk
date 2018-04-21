//! Traits for configuring devices.
//! 
//! TODO: Add Error result to trait

/// A trait used for configuring a device.
pub trait Configure<C: Default> {
    /// Returns the current configuration.
    fn config(&self) -> C;
    /// Sets the configuration to `cfg`.
    fn configure(&self, cfg: C) -> &Self;
    /// Sets the configuration to the result of a closure called with
    /// the default configuration as its argument.
    fn set_config<F: FnOnce(C) -> C>(&self, f: F) -> &Self {
        self.configure(f(C::default()))
    }
    /// Updates the configuration to the result of a closure called with
    /// the current configuration as its argument.
    fn with_config<F: FnOnce(C) -> C>(&self, f: F) -> &Self {
        self.configure(f(self.config()))
    }
}