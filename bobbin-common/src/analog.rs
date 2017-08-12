pub trait AnalogRead<T> {
    fn analog_read(&self) -> T;
}

pub trait AnalogWrite<T> {
    fn analog_write(&self, T) -> &Self;    
}