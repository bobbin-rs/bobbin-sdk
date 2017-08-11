pub trait AnalogInput<T> {
    fn analog_input(&self) -> T;
}

pub trait AnalogOutput<T> {
    fn analog_output(&self, T) -> &Self;    
}