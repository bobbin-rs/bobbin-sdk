// pub enum Direction {
//     Down,
//     Up,
// }

pub trait Timer<T> {
    fn stop(&self) -> &Self;
    fn running(&self) -> bool;

    fn period(&self) -> T;
    fn set_period(&self, value: T) -> &Self;

    fn counter(&self) -> T;

    fn timeout_flag(&self) -> bool;
    fn clr_timeout_flag(&self) -> &Self;

    fn wait_timeout_flag(&self) -> &Self {
        while !self.timeout_flag() {}
        self
    }    
}

pub trait Start<T> {
    fn start(&self, value: T) -> &Self;
}

pub trait StartDown<T> {
    fn start_down(&self, value: T) -> &Self;
}

pub trait StartUp<T> {
    fn start_up(&self, value: T) -> &Self;
}

pub trait StartDownOnce<T> {
    fn start_down_once(&self, value: T) -> &Self;
}

pub trait StartUpOnce<T> {
    fn start_up_once(&self, value: T) -> &Self;
}

pub trait Delay<T> {
    fn delay(&self, value: T) -> &Self;
}

pub trait Prescale<T> {
    fn prescale(&self) -> T;
    fn set_prescale(&self, value: T) -> &Self;
}

pub trait SetCounter<T> {
    fn set_counter(&self, value: T) -> &Self;
}

pub trait Compare<T> {
    fn compare(&self) -> T;
    fn set_compare(&self, value: T) -> &Self;

    fn compare_flag(&self) -> bool;
    fn clr_compare_flag(&self) -> &Self;

    fn wait_compare_flag(&self) -> &Self {
        while !self.compare_flag() {}
        self
    }
}