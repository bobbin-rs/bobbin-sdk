#![no_std]
#![feature(const_fn)]

extern crate bobbin_hz as hz;

#[macro_use]
mod macros;

pub use hz::Hz;

use core::fmt::Debug;

pub trait Id : Debug + Default + Clone + Copy {}

pub trait IdFor<T> {
    type Out: Id;
}

// pub trait Clock : Debug + Default + Clone + Copy  {
//     fn read() -> Hz;
// }

pub trait Clock {
    fn hz() -> Hz;
}


pub trait ClockFor<T> {
    fn clock_for(T) -> Hz;    
}

// pub trait Clock : Debug + Default + Clone + Copy  {
//     fn read() -> Hz;
//     fn read_hz(&self) -> Hz { Self::read() }
//     fn read_u32(&self) -> u32 { Self::read().into() }
// }

// pub trait ClockForId<ID: Id> { 
//     type Out: Clock; 
//     fn clock_for_id(&self, ID) -> Self::Out { Self::Out::default() }
//     fn hz_for_id(&self, ID) -> Hz { Self::Out::read() }
//     fn u32_for_id(&self, ID) -> u32 { Self::Out::read().into() }
// }

// pub trait ClockFor<T> {
//     type Out: Clock;
//     fn clock_for(&self, T) -> Self::Out { Self::Out::default() }
//     fn hz_for(&self, T) -> Hz { Self::Out::read() }
//     fn u32_for(&self, T) -> u32 { Self::Out::read().into() }
// }

// pub trait ClockTree : Debug + Default + Clone + Copy  {
//     type Defn;
//     type Impl;
// }

// impl<ID: Id, TI, TD, CT> ClockForId<ID> for CT
// where
//     Self: ClockTree<Defn=TD, Impl=TI>,
//     TI: ClockForId<ID>
// {
//     type Out = TI::Out;
// }    

// impl<T, TD, TI, CT> ClockFor<T> for CT
// where
//     Self: ClockTree<Defn=TD, Impl=TI>,
//     TD: IdFor<T>,
//     TI: ClockForId<TD::Out>,
// {
//     type Out = TI::Out;
// }
