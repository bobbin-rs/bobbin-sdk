//! Traits for managing Watchdog timers.

/// Unlock the watchdog timer.
pub trait Unlock {
    /// Unlock the Watchdog
    fn unlock(&self) -> &Self;
}

/// Lock the watchdog timer.
pub trait Lock {
    /// Lock the Watchdog
    fn lock(&self) -> &Self;
}

/// Enable the watchdog timer.
pub trait Enable {
    /// Enable the Watchdog
    fn enable(&self) -> &Self;
}

/// Disable the watchdog timer.
pub trait Disable {
    /// Disable the Watchdog
    fn disable(&self) -> &Self;
}

/// Refresh the watchdog timer
pub trait Refresh {
    /// Refresh the Watchdog timer, preventing it from firing
    fn refresh(&self) -> &Self;
}

/// Query the watchdog timer counter.
pub trait Counter<T> {
    /// Returns the watchdog counter
    fn counter(&self) -> T;
}

/// Update the watchdog timer counter.
pub trait SetCounter<T> {
    /// Sets the watchdog counter
    fn set_counter(&self, T) -> &Self;
}

/// Query the watchdog timeout period.
pub trait Timeout<T> {
    /// Returns the timeout period
    fn timeout(&self) -> T;
}

/// Set the watchdog timeout period.
pub trait SetTimeout<T> {
    /// Sets the timeout period
    fn set_timeout(&self, T) -> &Self;
}