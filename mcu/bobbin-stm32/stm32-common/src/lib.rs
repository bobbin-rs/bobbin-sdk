#![no_std]
#![allow(unused_unsafe)]

extern crate bobbin_common;
extern crate bobbin_cortexm;
pub mod chip;
pub mod hal;

#[cfg(test)]
mod tests;