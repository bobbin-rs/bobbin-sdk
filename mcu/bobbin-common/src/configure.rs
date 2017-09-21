pub trait Configure<C: Default> {
    fn config(&self) -> C;
    fn configure(&self, cfg: C) -> &Self;
    fn set_config<F: FnOnce(C) -> C>(&self, f: F) -> &Self {
        self.configure(f(C::default()))
    }

    fn with_config<F: FnOnce(C) -> C>(&self, f: F) -> &Self {
        self.configure(f(self.config()))
    }
}