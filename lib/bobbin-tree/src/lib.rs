#![no_std]
#![feature(const_fn)]

extern crate bobbin_hz as hz;

pub use hz::Hz;

use core::fmt::Debug;

pub trait Id : Debug + Default + Clone + Copy {}

pub trait IdFor<T> {
    type Out: Id;
}

pub trait Clock : Default {
    fn hz() -> Hz;
}

pub trait ClockFor<T> {
    fn clock_for(&self, T) -> Hz;    
}
