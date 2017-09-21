pub trait Enabled {
    fn enabled(&self) -> bool;
    fn set_enabled(&self, value: bool) -> &Self;
    fn enable(&self) -> &Self {
        self.set_enabled(true)
    }
    fn disable(&self) -> &Self {
        self.set_enabled(false)
    }
}