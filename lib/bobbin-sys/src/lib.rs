//! This crate provides services useful for building embedded applications.
#![no_std]
#![feature(asm, no_debug)]

#[cfg(feature="logger")]
extern crate log;

extern crate bobbin_mcu;
extern crate bobbin_hal;

#[macro_use]
mod macros;

pub mod prelude;
pub mod console;
pub mod heap;
pub mod tick;
pub mod pend;
pub mod irq_dispatch;
pub mod ring;
pub mod system;
pub mod board;
pub mod unwrap_or_abort;

#[cfg(feature="logger")]
pub mod logger;