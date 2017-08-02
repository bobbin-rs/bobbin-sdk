// pub enum Direction {
//     Down,
//     Up,
// }

pub trait Timer<T> {
    fn enabled(&self) -> bool;
    fn set_enabled(&self, bool) -> &Self;

    fn prescaler(&self) -> T;
    fn set_prescaler(&self, prescale: T) -> &Self;

    fn period(&self) -> T;
    fn set_period(&self, value: T) -> &Self;

    fn timeout(&self) -> bool;
    fn clr_timeout(&self) -> &Self;

    fn wait_timeout(&self) -> &Self {
        while !self.timeout() {}
        self
    }    
}

pub trait Delay<T> {
    fn delay(&self, reload: T, prescale: T) -> &Self;
}

pub trait Compare<T> {
    fn compare(&self) -> T;
    fn set_compare(&self, value: u16) -> &Self;

    fn compare_flag(&self) -> bool;
    fn clr_compare_flag(&self) -> &Self;

    fn wait_compare_flag(&self) -> &Self {
        while !self.compare_flag() {}
        self
    }
}