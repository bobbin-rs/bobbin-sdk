// pub enum Direction {
//     Down,
//     Up,
// }

pub trait Timer<T> {
    fn enabled(&self) -> bool;
    fn set_enabled(&self, bool) -> &Self;

    fn prescaler(&self) -> T;
    fn set_prescaler(&self, prescale: T) -> &Self;

    fn reload(&self) -> T;
    fn set_reload(&self, value: T) -> &Self;

    fn timeout(&self) -> bool;
    fn clr_timeout(&self) -> &Self;

    fn wait(&self) -> &Self {
        while !self.timeout() {}
        self
    }    
}

pub trait Delay<T> {
    fn delay(&self, reload: T, prescale: T) -> &Self;
}