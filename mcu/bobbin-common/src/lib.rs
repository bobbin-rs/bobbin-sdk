#![no_std]
#![feature(const_fn, core_intrinsics, allocator_api, alloc)]

#[cfg(not(target_os="none"))]
#[macro_use]
extern crate std;

// pub extern crate alloc;
pub extern crate bobbin_bits as bits;

#[macro_use] pub mod periph;
#[macro_use] pub mod pin;
#[macro_use] pub mod channel;
#[macro_use] pub mod irq;
#[macro_use] pub mod signal;

pub mod clock;
pub mod crc;
pub mod timer;
pub mod digital;
pub mod analog;
pub mod serial;
pub mod spi;
pub mod i2c;
pub mod can;
pub mod watchdog;
pub mod configure;
pub mod enabled;
pub mod reset;
pub mod ring;
// pub mod heap;

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
