#![no_std]
#![feature(repr_align, attr_literals)]

extern crate bobbin_common;
extern crate bobbin_cortexm;
pub mod chip;
pub mod hal;

#[cfg(test)]
mod tests;