pub trait AnalogRead<T> {
    fn start(&self) -> &Self;
    fn is_complete(&self) -> bool;
    fn clr_complete(&self) -> &Self;
    fn wait_complete(&self) -> &Self {
        while !self.is_complete() {}
        self
    }
    fn read(&self) -> T;
    fn analog_read(&self) -> T {
        self
            .start()
            .wait_complete()
            .read()
    }    
}

pub trait AnalogWrite<T> {
    fn analog_write(&self, T) -> &Self;    
}