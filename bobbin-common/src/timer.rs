// pub enum Direction {
//     Down,
//     Up,
// }

pub trait Timer {
    const MAX_PRESCALE: u32;
    const MAX_RELOAD: u32;
    const MAX_COUNT: u32;

    fn enabled(&self) -> bool;
    fn set_enabled(&self, bool) -> &Self;

    // fn dir(&self) -> Direction;
    // fn set_dir(&self, dir: Direction);

    fn prescaler(&self) -> u32;
    fn set_prescaler(&self, prescale: u32) -> &Self;

    fn reload(&self) -> u32;
    fn set_reload(&self, value: u32) -> &Self;

    fn counter(&self) -> u32;
    fn set_counter(&self, value: u32) -> &Self;

    fn overflow(&self) -> bool;
    fn clr_overflow(&self) -> &Self;

    fn sync(&self) -> &Self;

    fn wait(&self) -> &Self {
        self.clr_overflow();
        while !self.overflow() {}
        self
    }
    
    fn delay(&self, reload: u32, prescale: u32) -> &Self;
    
}