pub use chip::lptim::*;

pub trait LptimExt {
    // fn enabled(&self) -> bool;
    // fn set_enabled(&self, bool) -> &Self;
    // fn start_continous(&self) -> &Self;
    // fn start_single(&self) -> &Self;
    // fn compare(&self) -> u16;
    // fn set_compare(&self, u16) -> &Self;
    // fn auto_reload(&self) -> u16;
    // fn set_auto_reload(&self, u16) -> &Self;
    // fn counter(&self) -> u16;    
}

impl<T> LptimExt for Periph<T> {

}