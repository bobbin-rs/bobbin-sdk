//! A trait for enabling or disabling a device.

/// Used for querying the device enabled state to enable / disable the device.
/// 
/// See the implementation for details on exactly what it means for the 
/// device to be enabled or disabled.
pub trait Enabled {
    /// Returns true if the device is currently enabled.
    fn enabled(&self) -> bool;
    /// Sets the device enabled state to `value`.
    fn set_enabled(&self, value: bool) -> &Self;
    /// Enables the device.
    fn enable(&self) -> &Self {
        self.set_enabled(true)
    }
    /// Disables the device.
    fn disable(&self) -> &Self {
        self.set_enabled(false)
    }
}