#![no_std]
#![cfg_attr(target_os="none", feature(compiler_builtins_lib))]

#[cfg(target_os="none")] extern crate compiler_builtins;
pub extern crate bobbin_common;
extern crate bobbin_cortexm;
extern crate kinetis_common;

pub use bobbin_common as common;
pub mod chip;
pub mod hal;

#[cfg(test)]
mod tests;