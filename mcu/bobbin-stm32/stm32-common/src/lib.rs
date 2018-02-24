#![no_std]
#![allow(unused_unsafe)]

pub extern crate bobbin_cortexm;
extern crate bobbin_common;

pub mod chip;
pub mod hal;

pub use bobbin_cortexm::*;
pub use hal::*;

#[cfg(test)]
mod tests;