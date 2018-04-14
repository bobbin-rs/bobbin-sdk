#![no_std]

#[cfg(not(target_os="none"))]
#[macro_use]
extern crate std;

extern crate bobbin_hz as hz;
extern crate bobbin_tree as tree;
extern crate bobbin_bits as bits;

pub mod mcu;
pub mod clock;
pub mod periph;
pub mod pin;
pub mod channel;
pub mod irq;
pub mod signal;
pub mod gate;
pub mod owned;

pub use periph::*;
pub use pin::*;
pub use channel::*;
pub use irq::*;

#[cfg(not(target_os="none"))]
pub mod vm;

#[cfg(target_os="none")]
pub use core::ptr::{read_volatile, write_volatile};

#[cfg(not(target_os="none"))]
pub mod rw;

#[cfg(not(target_os="none"))]
pub use rw::*;

pub trait Mcu : Default {
    fn id(&self) -> &'static str;
}

pub trait Get<T> {
    fn get(&self) -> T;
}

pub trait GetPeriph<T> {
    fn get_periph(&self) -> T;
}

pub trait GetPeriphInstance<T> {
    fn get_periph_instance(&self, index: usize) -> Option<T>;
    fn get_periph_instance_count(&self) -> usize;
}