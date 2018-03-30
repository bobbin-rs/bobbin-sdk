#![no_std]
#![allow(unused_unsafe)]
#![feature(use_extern_macros)]

#[allow(unused_imports)]
#[macro_use] extern crate log;

pub extern crate bobbin_common;
pub extern crate bobbin_cortexm;

pub mod periph;
pub mod ext;

pub use bobbin_cortexm::*;
pub use ext::*;