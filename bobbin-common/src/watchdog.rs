pub trait Unlock {
    /// Unlock the Watchdog
    fn unlock(&self) -> &Self;
}

pub trait Lock {
    /// Lock the Watchdog
    fn lock(&self) -> &Self;
}

pub trait Refresh {
    /// Refresh the Watchdog timer
    fn refresh(&self) -> &Self;
}

pub trait Counter<T> {
    /// Returns the watchdog counter
    fn counter(&self) -> T;
}

pub trait SetCounter<T> {
    /// Sets the watchdog counter
    fn set_counter(&self, T) -> &Self;
}

pub trait Timeout<T> {
    /// Returns the timeout period
    fn timeout(&self) -> T;
}

pub trait SetTimeout<T> {
    /// Sets the timeout period
    fn set_timeout(&self, T) -> &Self;
}