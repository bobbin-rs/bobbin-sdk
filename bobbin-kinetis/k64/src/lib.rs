#![no_std]
#![cfg_attr(not(test), feature(compiler_builtins_lib))]

#[cfg(not(test))] extern crate compiler_builtins;
pub extern crate bobbin_common;
pub extern crate bobbin_cortexm;
pub extern crate kinetis_common;

pub use bobbin_common as common;
pub mod chip;
pub mod hal;