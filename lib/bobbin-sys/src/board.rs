//! Board traits

/// Board operations
pub trait Board {
    type System;
    /// Returns a string identifying the board.
    fn id(&self) -> &'static str;
    /// Returns a reference to the System that the board implements.
    fn sys(&self) -> &Self::System;
    /// Returns a mutable reference to the System that board implements.
    fn sys_mut(&mut self) -> &mut Self::System;
    /// Execute a closure with the board in Run mode (i.e. with interrupts enabled).
    fn run<T, F: FnMut(&Self)->T>(&mut self, mut f: F) -> T {
        f(&*self)
    }       
}