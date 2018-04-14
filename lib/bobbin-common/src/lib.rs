#![no_std]
#![feature(use_extern_macros, const_fn, core_intrinsics, allocator_api, alloc)]

#[cfg(not(target_os="none"))]
#[macro_use]
extern crate std;

pub extern crate bobbin_bits as bits;
pub extern crate bobbin_mcu as mcu;
pub extern crate bobbin_hz as hz;
pub extern crate bobbin_hal as hal;
pub extern crate bobbin_sys as sys;

#[macro_use]
mod macros;

pub use mcu::*;
pub mod board;

// pub mod delay;
pub mod configure;
pub mod enabled;
// pub mod ring;

pub trait En {
    fn en(&self) -> bits::U1;
    fn set_en<V: Into<bits::U1>>(&self, value: V);
}    
