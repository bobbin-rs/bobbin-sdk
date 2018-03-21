pub trait Periph: Default {
    fn id(&self) -> &'static str;
    fn base(&self) -> *mut u32;
    fn index(&self) -> usize;
    fn ord(&self) -> usize;
}

pub trait IntoPeriph {
    type Target;
    fn into_periph(&self) -> &'static Self::Target;
}

pub trait ClockEnabled : Periph {
    fn clock_enabled(&self) -> bool;
    fn set_clock_enabled(&self, value: bool);
    fn enable_clock(&self) { self.set_clock_enabled(true) }
    fn disable_clock(&self) { self.set_clock_enabled(true) }
}

pub trait Reset : Periph {
    fn clock_reset(&self) -> bool;
}