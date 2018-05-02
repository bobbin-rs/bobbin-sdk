//! Board traits

/// Board operations
pub trait Board {
    /// Returns a string identifying the board.
    fn id(&self) -> &'static str;
}