// pub enum Direction {
//     Down,
//     Up,
// }

pub trait Timer<T> {
    const MAX_PRESCALE: T;
    const MAX_RELOAD: T;
    const MAX_COUNT: T;

    fn enabled(&self) -> bool;
    fn set_enabled(&self, bool) -> &Self;

    // fn dir(&self) -> Direction;
    // fn set_dir(&self, dir: Direction);

    fn prescaler(&self) -> T;
    fn set_prescaler(&self, prescale: T) -> &Self;

    fn reload(&self) -> T;
    fn set_reload(&self, value: T) -> &Self;

    fn counter(&self) -> T;
    fn set_counter(&self, value: T) -> &Self;

    fn overflow(&self) -> bool;
    fn clr_overflow(&self) -> &Self;

    fn sync(&self) -> &Self;

    fn wait(&self) -> &Self {
        self.clr_overflow();
        while !self.overflow() {}
        self
    }    
}

pub trait Delay<T> {
    fn delay(&self, reload: T, prescale: T) -> &Self;
}