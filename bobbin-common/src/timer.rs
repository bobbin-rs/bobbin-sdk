// pub enum Direction {
//     Down,
//     Up,
// }

pub trait Timer<T> {
    fn enabled(&self) -> bool;
    fn set_enabled(&self, bool) -> &Self;

    fn period(&self) -> T;
    fn set_period(&self, value: T) -> &Self;

    fn counter(&self) -> T;
    fn set_counter(&self, value: T) -> &Self;

    fn timeout_flag(&self) -> bool;
    fn clr_timeout_flag(&self) -> &Self;

    fn wait_timeout_flag(&self) -> &Self {
        while !self.timeout_flag() {}
        self
    }    

    fn delay(&self, value: T) -> &Self {
        self
            .set_enabled(false)
            .set_period(value)
            .set_enabled(true)
            .clr_timeout_flag()
            .wait_timeout_flag()
            .set_enabled(false)
    }
}

pub trait Prescale<T> {
    fn prescale(&self) -> T;
    fn set_prescale(&self, prescale: T) -> &Self;
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