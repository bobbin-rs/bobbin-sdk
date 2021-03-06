//! `bobbin-mcu` defines a set of data types used to represent the internal
//! components and relationships within an embedded MCU. This includes:
//! 
//! - Registers
//! - Peripherals
//! - Pins
//! - Pin Assignments
//! - Channels
//! - Clocks
//! - Clock Gates
//! - Interrupts
//! 
//! These data types and macros are designed to be used by [bobbin-chip](../.../bobbin-chip/index.html)
//! to automatically generate MCU crates. They can also be used by HAL, driver and application crates
//! for compile-time checking and discovery of internal MCU relationships and metadata and applications
//! such as automatic IRQ lookup for drivers, dynamic clock introspection, ownership and reference counting, 
//! and other useful tasks.

#![no_std]
#![feature(const_fn, core_intrinsics)]

#[cfg(not(target_os="none"))]
#[macro_use]
extern crate std;

pub extern crate bobbin_hz as hz;
extern crate bobbin_bits as bits;
// pub extern crate bobbin_tree as tree;

#[macro_use]
mod macros;

pub mod prelude;

pub mod mcu;
pub mod clock;
pub mod register;
pub mod periph;
pub mod pin;
pub mod channel;
pub mod irq;
pub mod signal;
pub mod gate;
pub mod owned;

pub use register::*;
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
pub trait En {
    fn en(&self) -> bits::U1;
    fn set_en<V: Into<bits::U1>>(&self, value: V);
}    
