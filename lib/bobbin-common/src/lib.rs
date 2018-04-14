#![no_std]
#![feature(use_extern_macros, const_fn, core_intrinsics, allocator_api, alloc)]

#[cfg(not(target_os="none"))]
#[macro_use]
extern crate std;

pub extern crate bobbin_bits as bits;
pub extern crate bobbin_mcu as mcu;
pub extern crate bobbin_hz as hz;
pub extern crate bobbin_tree as tree;
pub extern crate bobbin_hal as hal;
pub extern crate bobbin_sys as sys;

#[macro_use]
mod macros;

// pub mod mcu;
pub mod periph;
pub mod pin;
pub mod channel;
pub mod irq;
pub mod signal;
pub mod gate;
pub mod board;
pub mod owned;
//pub mod dispatch;

pub mod clock;

pub mod delay;
pub mod configure;
pub mod enabled;
pub mod ring;
pub mod led;
pub mod btn;

pub use periph::*;
pub use pin::*;
pub use channel::*;
pub use irq::*;


#[cfg(not(target_os="none"))]
mod vm;

pub use core::ops::Deref;

#[cfg(target_os="none")]
pub use core::ptr::{read_volatile, write_volatile};

#[cfg(not(target_os="none"))]
pub mod rw;

#[cfg(not(target_os="none"))]
pub use rw::*;

pub trait En {
    fn en(&self) -> bits::U1;
    fn set_en<V: Into<bits::U1>>(&self, value: V);
}    