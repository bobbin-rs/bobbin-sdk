#![no_std]

#[cfg(feature="logger")]
extern crate log;

#[macro_use]
mod macros;

pub mod console;
pub mod heap;
pub mod memory;
pub mod ring;

#[cfg(feature="logger")]
pub mod logger;